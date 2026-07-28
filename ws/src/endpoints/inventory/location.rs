use tracing::{debug, error, info};

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;

use actix_web::{HttpResponse, Responder, http, web};

use crate::{
    classes::user,
    endpoints::{ApiResponse, default_option_response},
};

use inv_provider::{Location, LocationsProvider};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("save")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(location_save_post)),
    );
}

#[derive(Debug, Deserialize)]
struct LocationSavePost {
    warehouse_id: uuid::Uuid,
    location: Location,
}

async fn location_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<LocationSavePost>,
) -> impl Responder {
    info!("location_save_post");

    let provider = inv_provider_postgres::location::LocationsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match provider
        .location_save(&tenant_id, &params.warehouse_id, &params.location)
        .await
    {
        Err(e) => {
            error!("unable to save warehouse: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("Unable to save location"));
        }
        Ok(_) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "Location saved successfully",
                None,
            ));
        }
    }
}
