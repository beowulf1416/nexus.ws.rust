use tracing::{
    info,
    error,
    debug
};

use std::sync::Arc;
use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use http::header::AUTHORIZATION;
use actix_web::{
    guard,
    dev::ConnectionInfo, 
    http, 
    web, 
    HttpResponse, 
    Responder
};


use crate::{
    classes::{
        user,
        tenant
    },
    endpoints::{
        ApiResponse,
        default_option_response
    }
};

use auth_provider::AuthProvider;
use users_provider::UsersProvider;
use tenants_provider::TenantsProvider;





pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("sign-in")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(user_session_signin_post))
        )
        .service(
            web::resource("user")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(user_session_user_post))
        )
        .service(
            web::resource("tenant/set")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(user_session_tenant_set_post))
        )
    ;
}



#[derive(Debug, Deserialize)]
struct UserSessionSignInPost {
    email: String,
    pw: String
}



async fn user_session_signin_post(
    info: ConnectionInfo,
    // config: web::Data<Arc<config::Config>>,
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    tg: web::Data<Arc<token::TokenGenerator>>,
    params: web::Json<UserSessionSignInPost>
) -> impl Responder {
    info!("user_session_signin_post");

    let ap = auth_provider_postgres::PostgresAuthProvider::new(&dp);
    let authentic = match ap.authenticate_by_password(
        &params.email,
        &params.pw
    ).await {
        Err(e) => {
            error!("unable to authenticate user: {}", e);
            false
        }
        Ok(r) => {
            r
        }
    };

    let mut rb = HttpResponse::Ok();

    if authentic {
        let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

        let user = match up.fetch_by_email(&params.email).await {
            Err(e) => {
                error!("unable to fetch user record from email: {}", e);
                users_provider::User::nil()
            }
            Ok(u) => {
                u
            }
        };

        if !user.is_nil() {
            // generate jwt token
            // let tg = token::TokenGenerator::new(config.token_secret().as_str());

            let claim = token::Claim::new(
                &user.user_id,
                &uuid::Uuid::nil(),
                &params.email,
                &params.email
            );

            match tg.generate(
                &claim
            ) {
                Err(e) => {
                    error!("unable to generate token: {}", e);
                }
                Ok(token) => {
                    rb.append_header((http::header::AUTHORIZATION, format!("Bearer {}", token)));
                }
            }
        }
    }


    let response = rb.json(ApiResponse::new(
        authentic,
        if authentic { "user is authentic" } else { "user/password is not correct" },
        None
    ));

    return response;
}




#[derive(Debug, Serialize)]
struct UserSessionResponseData {
    name: String,
    tenant: tenant::Tenant,
    permissions: Vec<u16>,
    tenants: Vec<tenant::Tenant>
}


async fn user_session_user_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    // user: extractors::user::User
    user: user::User
) -> impl Responder {
    info!("user_session_user_post");

    debug!("{:?}", user);

    let user_id = user.user_id();
    let mut tenant = tenant::Tenant::default();
    let mut email = String::from("");

    let tenants = user.tenants();

    // let ap = auth_provider_postgres::PostgresAuthProvider::new(&dp);
    // if let Ok(auth_details) = ap.fetch_user_by_id(&user.user_id()).await {
    //     email = auth_details.email;
    // }

    let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

    // let f1 = ap.fetch_user_by_id(&user.user_id());
    // let f2 = tp.tenants_fetch_by_id(&user.tenant_id());

    // if let Ok(t) = tp.tenants_fetch_by_id(&user.tenant().tenant_id()).await {
    //     tenant = tenant::Tenant::new(
    //         &t.tenant_id(),
    //         &t.name(),
    //         &t.description()
    //     );
    // }

    if let Ok(t) = tp.tenants_fetch_by_id(&user.tenant().tenant_id()).await {
        tenant = tenant::Tenant::new(
            &t.tenant_id(),
            &t.name(),
            &t.description()
        );
    }


    return HttpResponse::Ok()
        .json(ApiResponse::new(
            true,
            "success",
            Some(json!({
                "user": UserSessionResponseData {
                    name: user.name(),
                    tenant: tenant,
                    permissions: vec!(),
                    tenants: tenants
                }
            }))
        ));
}



#[derive(Debug, Deserialize)]
struct UserSessionTenantSwitchPost {
    tenant_id: uuid::Uuid
}

async fn user_session_tenant_set_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    tg: web::Data<Arc<token::TokenGenerator>>,
    user: user::User,
    params: web::Json<UserSessionTenantSwitchPost>
) -> impl Responder {
    info!("user_session_tenant_set_post");

    let mut rb = HttpResponse::Ok();

    if user.is_authenticated() {
        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

        if let Ok(new_tenant) = tp.tenants_fetch_by_id(&params.tenant_id).await {
            let claim = token::Claim::new(
                &user.user_id(),
                &new_tenant.tenant_id(),
                &user.name(),
                &user.email()
            );

            match tg.generate(
                &claim
            ) {
                Err(e) => {
                    error!("unable to generate token: {}", e);
                }
                Ok(token) => {
                    rb.append_header((http::header::AUTHORIZATION, format!("Bearer {}", token)));
                }
            }
        }
    }

    let response = rb.json(ApiResponse::new(
        true,
        "switched to tenant",
        None
    ));

    return response;
}