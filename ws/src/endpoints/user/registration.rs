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

use rand::{
    distr::Alphanumeric,
    Rng
};

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

use user_registration::UserRegistrationProvider;
use users_provider::UsersProvider;
use auth_provider::AuthProvider;


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


#[derive(Debug, Serialize, Deserialize)]
struct UserRegistrationSignUpPost {
    id: uuid::Uuid,
    email: String
}

async fn user_registration_signup_post(
    mailer: web::Data<Arc<mailer::Mailer>>,
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UserRegistrationSignUpPost>
) -> impl Responder {
    info!("user_registration_signup_post");

    let ur = user_registration_postgres::PostgresUserRegistrationProvider::new(&dp);

    // let mut register_id = uuid::Uuid::nil(); 
    // match uuid::Uuid::parse_str(&params.id) {
    //     Ok(value) => {
    //         register_id = value;
    //     }
    //     Err(e) => {
    //         error!("Invalid UUID format for id: {}", e);
    //         return HttpResponse::BadRequest()
    //             .json(ApiResponse::error("invalid_uuid_format"))
    //             ;
    //     }
    // };

    // generate token
    let mut rng = rand::rng();
    let token: String = (0..50)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
        ;

    match ur.register_user(
        &params.id,
        &params.email,
        &token
    ).await {
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

    let ur = user_registration_postgres::PostgresUserRegistrationProvider::new(&dp);

    if let Err(e) = ur.verify_registration(
        &params.register_id, 
        &params.token).await {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("error while verifying registration"));
    }

    if let Err(e) = ur.fetch_registration_details_by_id(
        &params.register_id
    ).await {
        error!("unable to fetch user registration details: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("error while verifying registration"));
    }
    let urd = match ur.fetch_registration_details_by_id(&params.register_id).await {
        Ok(r) => {
            r
        }
        Err(e) => {
            error!("unable to fetch user registration details: {}", e);
            user_registration::UserRegistrationDetails::new(
                &params.register_id,
                "",
                ""
            )
        }
    };

    let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

    if let Err(e) = up.save(
        &params.register_id,
        "",
        "",
        "",
        "",
        ""
    ).await {
        error!("unable to save user details: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("error while verifying registration"));
    }

    let ap = auth_provider_postgres::PostgresAuthProvider::new(&dp);

    if let Err(e) = ap.add_user_auth_password(
        &params.register_id,
        urd.email().as_str(),
        &params.pw
    ).await {
        error!("unable to add user authentication via password: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("error while verifying registration"));
    }


    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"));
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

    // let ur = user_registration::UserRegistration::new(&dp);
    let ur = user_registration_postgres::PostgresUserRegistrationProvider::new(&dp);

    match ur.fetch_registration_details_by_token(&params.token).await {
        Ok(urd) => {
            debug!("user_registration_details_post ok");
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "success",
                    Some(json!({
                        "details": urd
                    }))
                ));
        }
        Err(e) => {
            error!("unable to retrieve user registration details: {:?}", e);
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}





// #[cfg(test)]
// mod tests {
//     use super::*;

//     use rand::*;
//     use actix_web::{
//         web
//     };
//     use serde::Serialize;
//     use serde_json::json;


//     #[actix_web::test]
//     async fn test_registration_endpoint() {

//         let ursp = UserRegistrationSignUpPost {
//             id: uuid::Uuid::new_v4(),
//             email: format!("test_{}@test.com", rand::random::<u16>())
//         };
//         let params = actix_web::web::Data::Json::new(ursp);

//         let cfg = config::Config::from_env();
//         let mailer = mailer::Mailer::new();
//         let dp = database_provider::DatabaseProvider::new(&cfg);


//         let r = user_registration_signup_post(
//             web::Data::new(std::sync::Arc::new(mailer)),
//             web::Data::new(std::sync::Arc::new(dp)),
//             params
//         ).await;
//     }
// }