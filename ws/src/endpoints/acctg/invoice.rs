use tracing::{debug, error, info};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

// use http::header::AUTHORIZATION;
use actix_web::{HttpResponse, Responder, guard, http, web};

use crate::{
    classes::{
        user,
        // tenant,
        // permission
    },
    endpoints::{ApiResponse, default_option_response},
};

use acctg_provider::invoice::InvoiceProvider;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("invoice/types/fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(invoice_types_fetch_post),
            ),
    );
}

async fn invoice_types_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
) -> impl Responder {
    info!("invoice_types_fetch_post");

    return HttpResponse::Ok().json(ApiResponse::ok("todo"));
}
