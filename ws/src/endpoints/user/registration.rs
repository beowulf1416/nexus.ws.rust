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
            web::resource("")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(user_registration_signup_post))
        )
        .service(
            web::resource("verified")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(user_registration_signup_verified_post))
        )
    ;
}


#[derive(Debug, Deserialize)]
struct UserRegistrationSignUpPost {
    email: String
}

async fn user_registration_signup_post(
    params: web::Json<UserRegistrationSignUpPost>
) -> impl Responder {
    info!("user_registration_signup_post");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}


#[derive(Debug, Deserialize)]
struct UserRegistrationSignUpVerifiedPost {
    token: String,
    pw: String
}

async fn user_registration_signup_verified_post(
    params: web::Json<UserRegistrationSignUpVerifiedPost>
) -> impl Responder {
    info!("user_registration_signup_verified_post");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}