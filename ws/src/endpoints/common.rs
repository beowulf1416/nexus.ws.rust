use tracing::{debug, error, info};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use actix_web::{HttpResponse, Responder, guard, http, web};

use crate::{
    classes::{tenant, user},
    endpoints::{ApiResponse, default_option_response},
};

use commons_provider::CommonsProvider;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("countries")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(countries_fetch_post)),
    )
    .service(
        web::resource("currencies")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(currencies_fetch_post)),
    )
    .service(
        web::resource("dimensions")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(dimensions_fetch_post)),
    )
    .service(
        web::resource("uoms")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(uoms_fetch_post)),
    )
    .service(
        web::resource("uoms/dimension")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(uoms_dimension_fetch_post)),
    );
}

async fn countries_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
) -> impl Responder {
    info!("countries_fetch_post");

    let cp = commons_provider_postgres::PostgresCommonsProvider::new(&dp);
    match cp.fetch_countries().await {
        Err(e) => {
            error!("unable to fetch countries: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch countries"));
        }
        Ok(countries) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully retrieved countries",
                Some(json!({
                    "countries": countries
                })),
            ));
        }
    }
}

async fn currencies_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
) -> impl Responder {
    info!("currencies_fetch_post");

    let cp = commons_provider_postgres::PostgresCommonsProvider::new(&dp);
    match cp.fetch_currencies().await {
        Err(e) => {
            error!("unable to fetch currencies: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch currencies"));
        }
        Ok(currencies) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully retrieved currencies",
                Some(json!({
                    "currencies": currencies
                })),
            ));
        }
    }
}

async fn dimensions_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
) -> impl Responder {
    info!("dimensions_fetch_post");

    let cp = commons_provider_postgres::PostgresCommonsProvider::new(&dp);
    match cp.fetch_dimensions().await {
        Err(e) => {
            error!("unable to fetch dimensions: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch dimensions"));
        }
        Ok(dimensions) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully retrieved dimensions",
                Some(json!({
                    "dimensions": dimensions
                })),
            ));
        }
    }
}

async fn uoms_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
) -> impl Responder {
    info!("uoms_fetch_post");

    let cp = commons_provider_postgres::PostgresCommonsProvider::new(&dp);
    match cp.fetch_uoms().await {
        Err(e) => {
            error!("unable to fetch uoms: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch uoms"));
        }
        Ok(uoms) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully retrieved uoms",
                Some(json!({
                    "uoms": uoms
                })),
            ));
        }
    }
}

#[derive(Debug, Deserialize)]
struct UomsDimensionFetchPost {
    dimension_id: i16,
}

async fn uoms_dimension_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<UomsDimensionFetchPost>,
) -> impl Responder {
    info!("uoms_dimension_fetch_post");

    let cp = commons_provider_postgres::PostgresCommonsProvider::new(&dp);
    match cp.fetch_uoms_by_dimension_id(&params.dimension_id).await {
        Err(e) => {
            error!("unable to fetch uoms: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch uoms"));
        }
        Ok(uoms) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully retrieved uoms",
                Some(json!({
                    "uoms_dimension": uoms
                })),
            ));
        }
    }
}
