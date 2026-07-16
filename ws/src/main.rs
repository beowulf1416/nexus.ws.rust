#![deny(clippy::unwrap_used)]
#![allow(clippy::needless_return)]

// extern crate tracing;

mod classes;
mod endpoints;
mod extractors;
mod guards;
mod middleware;

use tracing::{debug, error, info};
use tracing_subscriber::FmtSubscriber;

use serde::Deserialize;
use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};

use actix_web::{App, HttpResponse, HttpServer, error, web};

use database_provider::DatabaseProvider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();

    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(tracing::Level::TRACE)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("setting default subscriber failed");

    tracing_subscriber::fmt::init();

    info!("Starting up ...");

    let cfg = config::Config::from_env();
    debug!("config: {:?}", cfg);

    let db_provider = database_provider::DatabaseProvider::new(&cfg);

    let token_generator = token::TokenGenerator::new(&cfg.token_secret());

    let mut http_server = HttpServer::new(move || {
        let app = App::new()
            .wrap(actix_web::middleware::from_fn(
                crate::middleware::cors::cors_middleware,
            ))
            .wrap(actix_web::middleware::from_fn(
                crate::middleware::auth::auth_middleware,
            ))
            .app_data(web::Data::new(Arc::new(cfg.clone())))
            .app_data(web::Data::new(Arc::new(mailer::Mailer::new())))
            .app_data(web::Data::new(Arc::new(db_provider.clone())))
            .app_data(web::Data::new(Arc::new(token_generator.clone())))
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error!("JSON PARSE ERROR: {}", err);

                let error_details = err.to_string();

                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(
                        serde_json::json!({ "error": "Invalid JSON", "details": error_details }),
                    ),
                )
                .into()
            }))
            .service(web::scope("/api/v1/common").configure(crate::endpoints::common::config))
            .service(web::scope("/api/v1/session").configure(crate::endpoints::session::config))
            .service(
                web::scope("/api/v1/user/sign-up")
                    .configure(crate::endpoints::user::registration::config),
            )
            .service(web::scope("/api/v1/users").configure(crate::endpoints::user::users::config))
            .service(
                web::scope("/api/v1/permissions").configure(crate::endpoints::permissions::config),
            )
            .service(
                web::scope("/api/v1/admin/tenants")
                    .configure(crate::endpoints::admin::tenants::config),
            )
            .service(
                web::scope("/api/v1/admin/users").configure(crate::endpoints::admin::users::config),
            )
            // .service(web::scope("/documents").configure(crate::endpoints::documents::config))
            .service(web::scope("/api/v1/file").configure(crate::endpoints::file::config))
            .service(
                web::scope("/api/v1/acctg/accounts")
                    .configure(crate::endpoints::acctg::accounts::config),
            )
            .service(
                web::scope("/api/v1/acctg/invoices")
                    .configure(crate::endpoints::acctg::invoice::config),
            )
            .service(web::scope("/api/v1/crm").configure(crate::endpoints::crm::config))
            .service(
                web::scope("/api/v1/inv/warehouses")
                    .configure(crate::endpoints::inventory::warehouse::config),
            )
            .service(
                web::scope("/api/v1/inv/items")
                    .configure(crate::endpoints::inventory::item::config),
            );

        return app;
    })
    .workers(2);

    http_server = http_server.bind("localhost:8080")?;

    info!("Starting server...");

    let server = http_server.run();
    return server.await;
}
