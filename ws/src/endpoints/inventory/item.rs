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

use inv_provider::InventoryProvider;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(item_save_post))
        )
    ;
}


#[derive(Debug, Deserialize)]
struct ItemSavePost {
    tenant_id: uuid::Uuid,
    id: uuid::Uuid,
    active: bool,
    created: chrono::DateTime<chrono::Utc>,
    name: String,
    description: String,
    sku: String,
    upc: String
}

async fn item_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<ItemSavePost>
) -> impl Responder {
    info!("item_save_post");

    let ip = inv_provider_postgres::PostgresInventoryProvider::new(&dp);

    let item = inv_provider::Item {
        id: params.id,
        active: params.active,
        created: params.created,
        name: params.name.clone(),
        description: params.description.clone(),
        sku: params.sku.clone(),
        upc: params.upc.clone()
    };

    match ip.item_save(
        &params.tenant_id,
        &item
    ).await {
        Err(e) => {
            error!("unable to save item: {:?}", e);
            return HttpResponse::InternalServerError().json(
                ApiResponse::error("Unable to save item")
            );
        },
        Ok(_) => {
            return HttpResponse::Ok().json(
                ApiResponse::ok("Item saved successfully")
            );
        }
    }
}