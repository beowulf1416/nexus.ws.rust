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
                .route(web::post().to(admin_users_fetch))
        )
    ;
}



async fn admin_users_fetch(

) -> impl Responder {
    info!("admin_users_fetch");

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}