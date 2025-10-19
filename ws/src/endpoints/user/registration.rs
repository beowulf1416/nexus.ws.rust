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
    id: String,
    email: String
}

async fn user_registration_signup_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UserRegistrationSignUpPost>
) -> impl Responder {
    info!("user_registration_signup_post");

    let ur = user_registration::UserRegistration::new(&dp);

    let mut register_id = uuid::Uuid::nil(); 
    match uuid::Uuid::parse_str(&params.id) {
        Ok(value) => {
            register_id = value;
        }
        Err(e) => {
            error!("Invalid UUID format for id: {}", e);
            return HttpResponse::BadRequest()
                .json(ApiResponse::error("invalid_uuid_format"))
                ;
        }
    };

    match ur.register_user(&register_id, &params.email).await {
        Ok(_) => {
            info!("User registered successfully");
        },
        Err(e) => {
            error!("Error registering user: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("registration failed"))
                ;
        }   
    }

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