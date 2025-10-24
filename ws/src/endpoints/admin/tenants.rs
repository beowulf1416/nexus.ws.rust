use tracing::{
    info,
    debug,
    error
};
use std::sync::Arc;

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
                .route(web::post().to(admin_tenants_fetch))
        )
        .service(
            web::resource("save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(admin_tenants_save))
        )
    ;
}



async fn admin_tenants_fetch(

) -> impl Responder {
    info!("admin_tenants_fetch");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}



async fn admin_tenants_save(

) -> impl Responder {
    info!("admin_tenants_save");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}