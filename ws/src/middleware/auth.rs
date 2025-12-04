use tracing::{
    info,
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


use users_provider::UsersProvider;

// use crate::{classes::user, extractors};
use crate::classes::user;


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

            if let Ok(user) = up.fetch_by_id(&user_id).await {
                let u = user::User::new(
                    &user_id,
                    &tenant_id,
                    &user.email
                );

                return u;
            }
        }
    }

    return user::User::anonymous();
}


