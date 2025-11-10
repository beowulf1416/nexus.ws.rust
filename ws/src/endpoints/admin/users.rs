use tracing::{
    info,
    debug,
    error
};

use std::sync::Arc;
use serde::Deserialize;
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





pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(admin_users_fetch))
        )
        .service(
            web::resource("save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(admin_users_save))
        )
    ;
}



#[derive(Debug, Deserialize)]
struct UsersFetchPost {
    filter: String
}


async fn admin_users_fetch(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UsersFetchPost>
) -> impl Responder {
    info!("admin_users_fetch");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}



#[derive(Debug, Deserialize)]
struct UserSavePost {
    user_id: uuid::Uuid,
    email: String
}


async fn admin_users_save(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UserSavePost>
) -> impl Responder {
    info!("admin_users_save");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}