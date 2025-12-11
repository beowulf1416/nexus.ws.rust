use tracing::{
    info,
    error,
    debug
};
use std::sync::Arc;
use actix_web::{
    web,
    HttpMessage,
    body::MessageBody,
    dev::{
        ServiceRequest,
        ServiceResponse
    },
    middleware::Next,
    Error
};
use actix_http::{
    Method,
    header
};
use futures::try_join;


use users_provider::UsersProvider;
use tenants_provider::TenantsProvider;

// use crate::{classes::user, extractors};
use crate::classes::{
    permission,
    user,
    tenant
};


pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    info!("auth_middleware");

    let u = get_user_from_request(&req).await;
    req.extensions_mut().insert(u);

    // get route configuration

    let res = next.call(req).await?;

    return Ok(res);
}


async fn get_user_from_request(
    req: &ServiceRequest
) -> user::User {
    info!("get_user_from_request");

    if req.method() != Method::POST {
        return user::User::anonymous();
    }

    if let Some(header_value) = req.headers().get(header::AUTHORIZATION)
        && let Ok(token_value) = header_value.to_str() 
    {
        let pattern = regex::Regex::new(r"(?i)bearer").expect("incorrect regex pattern to retrieve bearer authentication");
        let token = pattern.replace(token_value, "").to_string();
        let token = token.trim();
        
        let mut user_id = uuid::Uuid::nil();
        let mut tenant_id = uuid::Uuid::nil();

        if let Some(tg) = req.app_data::<web::Data<Arc<token::TokenGenerator>>>() {
            let claim = tg.claim(&token);
            if !claim.is_empty() {
                user_id = claim.user_id;
                tenant_id = claim.tenant_id;
            }
        }

        if !user_id.is_nil() && let Some(dp_ref) = req.app_data::<web::Data<Arc<database_provider::DatabaseProvider>>>() {
            let dp = dp_ref.get_ref();
            let up = users_provider_postgres::PostgresUsersProvider::new(&dp);
            let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

            let f1 = up.fetch_by_id(&user_id);
            let f2 = tp.tenant_user_tenants_fetch(&user_id);
            let f3 = tp.tenants_fetch_by_id(&tenant_id);
            let f4 = tp.tenant_user_permissions_fetch(&user_id, &tenant_id);
            
            match try_join!(f1, f2, f3, f4) {
                Err(e) => {
                    error!("unable to fetch user or tenant data for user: {:?}", e);
                }
                Ok((user, tenants, tenant, permissions)) => {
                    let ts: Vec<tenant::Tenant> = tenants.iter().map(|t| {
                        let tenant_id = t.tenant_id();
                        let name = t.name();
                        let description = t.description();

                        return tenant::Tenant::new(
                            &tenant_id,
                            &name,
                            &description
                        );
                    }).collect();

                    let ps: Vec<permission::Permission> = permissions.iter().map(|p| {
                        let permission = p.id();
                        let name = p.name();

                        return permission::Permission::new(
                            &permission,
                            &name
                        );
                    }).collect();

                    let t = tenant::Tenant::new(
                        &tenant.tenant_id(),
                        &tenant.name(),
                        &tenant.description()
                    );

                    let u = user::User::new(
                        &user_id,
                        &t,
                        &user.email,
                        &user.email,
                        &ts,
                        &ps
                    );

                    debug!("returning authenticated user: {:?}", u);
                    return u;
                }
            }
        }
    }

    debug!("returning anonymous");
    return user::User::anonymous();
}


