use tracing::{
    info,
    error
};

use std::sync::Arc;
use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use actix_web::{
    dev::ConnectionInfo, 
    http, 
    web, 
    HttpResponse, 
    Responder
};


use crate::endpoints::{
    ApiResponse,
    default_option_response
};

use auth_provider::AuthProvider;





pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("sign-in")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(user_session_signin_post))
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
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
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

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            authentic,
            if authentic { "user is authentic" } else { "user/password is not correct" },
            None
        ));
}