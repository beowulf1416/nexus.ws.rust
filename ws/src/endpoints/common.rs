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
    guard,
    http, 
    web, 
    HttpResponse, 
    Responder
};

use crate::{
    classes::{
        user,
        tenant
    },
    endpoints::{
        ApiResponse,
        default_option_response
    }
};

use commons_provider::CommonsProvider;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("countries")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(countries_fetch_post))
        )
        .service(
            web::resource("currencies")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(currencies_fetch_post))
        )
    ;
}



async fn countries_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>
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
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved countries",
                    Some(json!({
                        "countries": countries
                    }))
                ));
        }
    }
}


async fn currencies_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>
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
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved currencies",
                    Some(json!({
                        "currencies": currencies
                    }))
                ));
        }
    }
}