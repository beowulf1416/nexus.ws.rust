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
    dev::ConnectionInfo, 
    http, 
    web, 
    HttpResponse, 
    Responder
};


use crate::{endpoints::{
    ApiResponse,
    default_option_response
}, extractors};

use auth_provider::AuthProvider;
use users_provider::UsersProvider;





pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("sign-in")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(user_session_signin_post))
        )
        .service(
            web::resource("user")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(user_session_user_post))
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

    if (authentic) {
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
    tenant: String,
    permissions: Vec<u16>
}


async fn user_session_user_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: extractors::user::User
) -> impl Responder {
    info!("user_session_user_post");

    debug!("{:?}", user);

    let user_id = user.user_id();
    let mut email = String::from("");

    let ap = auth_provider_postgres::PostgresAuthProvider::new(&dp);
    if let Ok(auth_details) = ap.fetch_user_by_id(&user.user_id()).await {
        email = String::from(auth_details.email);
    }

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            true,
            "success",
            Some(json!({
                "user": UserSessionResponseData {
                    name: email,
                    tenant: String::from("todo"),
                    permissions: vec!()
                }
            }))
        ));
}