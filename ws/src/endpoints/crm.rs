use tracing::{
    info,
    error,
    debug
};

use std::sync::Arc;
use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use http::header::AUTHORIZATION;
use actix_web::{
    guard,
    http,
    web,
    HttpResponse,
    Responder
};

use crate::{
    classes::{
        user,
        // tenant,
        // permission
    },
    endpoints::{
        ApiResponse,
        default_option_response
    }
};



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("person/save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(person_save_post))
        )
        .service(
            web::resource("business/save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(business_save_post))
        )
    ;
}


#[derive(Debug, Serialize)]
struct PersonSavePostData {
	tenant_id: String,
	person_id: String,

	first_name: String,
    middle_name: String,
    last_name: String,
    prefix: String,
    suffix: String,
    gender: String,
}

async fn person_save_post(
	user: user::User
) -> impl Responder {
	info!("person_save_post");

	return HttpResponse::Ok().json(
		ApiResponse::ok("person_save_post")
	);
}

async fn business_save_post(
	user: user::User
) -> impl Responder {
	info!("business_save_post");

	return HttpResponse::Ok().json(
		ApiResponse::ok("business_save_post")
	);
}
