use tracing::{debug, error, info};

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;

use actix_web::{HttpResponse, Responder, http, web};

use crate::{
    classes::user,
    endpoints::{ApiResponse, default_option_response},
};

use inv_provider::transactions::purchase_order::PurchaseOrderProvider;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("save")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(purchase_order_save_post)),
    );
}

#[derive(Debug, Deserialize)]
struct PurchaseOrderSavePost {
    purchase_order: inv_provider::transactions::purchase_order::PurchaseOrder,
}

async fn purchase_order_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<PurchaseOrderSavePost>,
) -> impl Responder {
    info!("purchase_order_save_post");

    let ppp =
        inv_provider_postgres::transactions::purchase_order::PurchaseOrderProviderPostgres::new(
            &dp,
        );

    let tenant_id = user.tenant().tenant_id();
    match ppp.save(&tenant_id, &params.purchase_order).await {
        Err(e) => {
            error!("unable to save purchase order: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::error(e));
        }
        Ok(_) => HttpResponse::Ok().json(ApiResponse::ok("purchase order saved successfully")),
    }
}
