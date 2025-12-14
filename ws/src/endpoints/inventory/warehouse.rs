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
                .route(web::post().to(warehouse_save_post))
        )
    ;
}



#[derive(Debug, Deserialize)]
struct WarehouseSavePost {
    tenant_id: uuid::Uuid,
    id: uuid::Uuid,
    name: String,
    address: String,
}


async fn warehouse_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<WarehouseSavePost>
) -> impl Responder {
    info!("warehouse_save_post");

    let provider = inv_provider_postgres::PostgresInventoryProvider::new(&dp);

    // match provider.warehouse_save(&params).await {
    //     Ok(_) => {
    //         HttpResponse::Ok().json(
    //             ApiResponse::<()> {
    //                 success: true,
    //                 message: Some("Warehouse saved successfully".to_string()),
    //                 data: None
    //             }
    //         )
    //     },
    //     Err(e) => {
    //         error!("unable to save warehouse: {:?}", e);
    //         HttpResponse::InternalServerError().json(
    //             ApiResponse::<()> {
    //                 success: false,
    //                 message: Some("Unable to save warehouse".to_string()),
    //                 data: None
    //             }
    //         )
    //     }
    // }

    return HttpResponse::InternalServerError().json(
        ApiResponse::error("Warehouse saving not yet implemented")
    );
}