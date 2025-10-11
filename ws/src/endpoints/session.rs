use log::{
    info
};

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
    params: web::Json<UserSessionSignInPost>
) -> impl Responder {
    info!("user_session_signin_post");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}