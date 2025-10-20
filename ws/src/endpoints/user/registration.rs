use tracing::{
    info,
    debug,
    error
};
use std::sync::Arc;

use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use rand::Rng;

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


const TOKEN_LENGTH: usize = 32;



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
        .service(
            web::resource("details")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(user_registration_details_post))
        )
    ;
}


#[derive(Debug, Deserialize)]
struct UserRegistrationSignUpPost {
    id: String,
    email: String
}

async fn user_registration_signup_post(
    mailer: web::Data<Arc<mailer::Mailer>>,
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

    // generate token
    let token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(TOKEN_LENGTH)
        .map(char::from)
        .collect()
        ;

    match ur.register_user(&register_id, &params.email, &token).await {
        Ok(_) => {
            info!("User registered successfully");

            // send email with link to verify email address
            if let Err(result) = mailer.send(format!("Please verify your email address by clicking the following link: /user/sign-up/verified/{}", token)) {
                error!("Error sending verification email: {}", result);
                return HttpResponse::InternalServerError()
                    .json(ApiResponse::error("email_sending_failed"))
                    ;
            }
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
    register_id: uuid::Uuid,
    token: String,
    pw: String
}

async fn user_registration_signup_verified_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UserRegistrationSignUpVerifiedPost>
) -> impl Responder {
    info!("user_registration_signup_verified_post");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}



#[derive(Debug, Deserialize)]
struct UserRegistrationDetailsPost {
    token: String
}


async fn user_registration_details_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UserRegistrationDetailsPost>
) -> impl Responder {
    info!("user_registration_details_post");

    let ur = user_registration::UserRegistration::new(&dp);
    match ur.get_details(&params.token).await {
        Ok(_) => {
            debug!("user_registration_details_post ok");
        }
        Err(e) => {
            error!("unable to retrieve user registration details: {:?}", e);
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}