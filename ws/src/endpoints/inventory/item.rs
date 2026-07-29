use tracing::{debug, error, info};

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;

use actix_web::{HttpResponse, Responder, http, web};

use crate::{
    classes::user,
    endpoints::{ApiResponse, default_option_response},
};

use inv_provider::ItemProvider;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("save")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(item_save_post)),
    )
    .service(
        web::resource("fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(items_fetch_post)),
    );
}

#[derive(Debug, Deserialize)]
struct ItemSavePost {
    item_id: uuid::Uuid,
    active: bool,
    version: i32,
    // created: chrono::DateTime<chrono::Utc>,
    // updated: chrono::DateTime<chrono::Utc>,
    name: String,
    description: String,
    sku: String,
    upc: String,

    perishable: bool,
    hazardous: bool,
    flammable: bool,
    esd_sensitive: bool,
}

async fn item_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<ItemSavePost>,
) -> impl Responder {
    info!("item_save_post");

    let ipp = inv_provider_postgres::item::ItemProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    let item = inv_provider::Item {
        item_id: params.item_id,
        active: params.active,
        version: params.version,
        created: chrono::Utc::now(),
        updated: chrono::Utc::now(),

        name: params.name.clone(),
        description: params.description.clone(),
        sku: params.sku.clone(),
        upc: params.upc.clone(),

        perishable: params.perishable,
        hazardous: params.hazardous,
        flammable: params.flammable,
        esd_sensitive: params.esd_sensitive,
    };

    match ipp.item_save(&tenant_id, &item).await {
        Err(e) => {
            error!("unable to save item: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("Unable to save item"));
        }
        Ok(_) => {
            return HttpResponse::Ok().json(ApiResponse::ok("Item saved successfully"));
        }
    }
}

#[derive(Debug, Deserialize)]
struct ItemsFetchPost {
    filter: String,
}

async fn items_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<ItemsFetchPost>,
) -> impl Responder {
    info!("items_fetch_post");

    let ipp = inv_provider_postgres::item::ItemProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();
    let filter = format!("%{}%", params.filter);

    match ipp.items_fetch(&tenant_id, &filter).await {
        Err(e) => {
            error!("unable to fetch items: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("Unable to fetch items"));
        }
        Ok(items) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "Items fetched successfully",
                Some(json!({
                    "items": items
                })),
            ));
        }
    }
}
