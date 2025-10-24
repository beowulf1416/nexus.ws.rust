// extern crate tracing;

mod extractors;
mod middleware;
mod endpoints;

use tracing::{
    info,
    error,
    debug
};
use tracing_subscriber::FmtSubscriber;

use std::{collections::HashMap, hash::Hash};
use std::sync::Arc;
use serde::Deserialize;

use actix_web::{
    web,
    App,
    HttpServer
};


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

    let mut http_server = HttpServer::new(move || {
        let app = App::new()
            .wrap(actix_web::middleware::from_fn(crate::middleware::cors::cors_middleware))

            .app_data(web::Data::new(Arc::new(cfg.clone())))
            .app_data(web::Data::new(Arc::new(mailer::Mailer::new())))
            .app_data(web::Data::new(Arc::new(db_provider.clone())))


            .service(web::scope("/session").configure(crate::endpoints::session::config))
            .service(web::scope("/user/sign-up").configure(crate::endpoints::user::registration::config))

            .service(web::scope("/admin/tenants").configure(crate::endpoints::admin::tenants::config))
            .service(web::scope("/admin/users").configure(crate::endpoints::admin::users::config))
        ;

        return app;
    })
    .workers(2)
    ;

    http_server = http_server.bind("localhost:8080")?;

    info!("Starting server...");

    let server = http_server.run();
    return server.await;
}
