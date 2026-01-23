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

use inv_provider::WarehouseProvider;



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
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
    country_id: i32
}



#[derive(Debug, Deserialize)]
struct WarehouseSavePost {
    tenant_id: uuid::Uuid,
    warehouse_id: uuid::Uuid,
    name: String,
    description: String,
    address: Address
}


async fn warehouse_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<WarehouseSavePost>
) -> impl Responder {
    info!("warehouse_save_post");
    // debug!("params", params);

    let provider = inv_provider_postgres::warehouse::PostgresWarehouseProvider::new(&dp);

    match provider.warehouse_save(
    	&params.tenant_id,
    	&inv_provider::Warehouse {
     		id: params.warehouse_id,
       		name: params.name.clone(),
         	description: params.description.clone(),
          	address: inv_provider::Address {
           		street: params.address.street.clone(),
             	city: params.address.city.clone(),
              	state: params.address.state.clone(),
               	zip_code: params.address.zip.clone(),
                country_id: params.address.country_id
           }
     	}
    ).await {
        Ok(_) => {
            return HttpResponse::Ok().json(
                ApiResponse::new(
                	true,
                 	"Warehouse saved successfully",
                 	None
                )
            );
        },
        Err(e) => {
            error!("unable to save warehouse: {:?}", e);
            return HttpResponse::InternalServerError().json(
                ApiResponse::error("Unable to save warehouse")
            );
        }
    }
}
