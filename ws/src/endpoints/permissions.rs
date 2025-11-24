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




use crate::{endpoints::{
    ApiResponse,
    default_option_response
}, extractors};

use permissions_provider::PermissionsProvider;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(permissions_fetch_post))
        )
    ;
}



#[derive(Debug, Deserialize)]
struct PermissionsFetchPost {
    filter: String
}


async fn permissions_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<PermissionsFetchPost>
) -> impl Responder {
    info!("permissions_fetch_post");

    let pp = permissions_provider_postgres::PostgresPermissionsProvider::new(&dp);

    match pp.fetch(
        &params.filter
    ).await {
        Err(e) => {
            error!("unable to fetch permissions: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch permissions"));
        }
        Ok(permissions) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully fetched permissions",
                    Some(json!({
                        "permissions": permissions
                    })
                )));
        }
    }
}