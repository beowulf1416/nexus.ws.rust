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


use actix_web::{
    http, 
    web, 
    HttpResponse, 
    Responder
};

use crate::endpoints::{
    ApiResponse,
    default_option_response
};

use users_provider::UsersProvider;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("active/set")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(users_set_active_post))
        )
        .service(
            web::resource("password/set")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(users_set_password_post))
        )
        .service(
            web::resource("search")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(users_search_post))
        )
    ;
}



#[derive(Debug, Deserialize)]
struct UserSetActivePost {
    user_id: uuid::Uuid,
    active: bool
}


// #[allow(dead_code)]
async fn users_set_active_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UserSetActivePost>
) -> impl Responder {
    info!("users_set_active_post");

    let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

    if let Err(e) = up.set_active(
        &params.user_id,
        &params.active
    ).await {
        error!("unable to set user active status: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("unable to set user active status"));
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}




#[derive(Debug, Deserialize)]
struct UsersSetPasswordPost {
    user_id: uuid::Uuid,
    password: String
}



async fn users_set_password_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UsersSetPasswordPost>
) -> impl Responder {
    info!("users_set_password_post");

    let ap = auth_provider_postgres::PostgresAuthProvider::new(&dp);

    


    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}




#[derive(Debug, Deserialize)]
struct UsersSearchPost {
    filter: String
}

async fn users_search_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UsersSearchPost>
) -> impl Responder {
    info!("users_search_post");

    let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}