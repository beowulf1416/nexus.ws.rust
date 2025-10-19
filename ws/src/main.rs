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



#[derive(Debug, Deserialize)]
struct EnvConfig {
    http_port: Option<u16>,
    cn: Option<String>,
}

#[derive(Debug, Clone)]
struct AppConfig {
    http_port: u16,
    cn: HashMap<String, String>,
}


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


    let cfg: AppConfig = match envy::from_env::<EnvConfig>() {
        Ok(config) => {
            debug!("Configuration: {:?}", config);
            match config.cn {
                Some(cn) => {
                    let mut connection_strings: HashMap<String, String> = HashMap::new();
                    
                    let kvs: Vec<&str> = cn.split(",").collect();
                    for kv in kvs.iter() {
                        let pair: Vec<&str> = kv.split("=").collect();
                        connection_strings.insert(pair[0].to_string(), pair[1].to_string());
                    }

                    let cfg = AppConfig {
                        http_port: config.http_port.unwrap_or(80),
                        cn: connection_strings.clone(),
                    };

                    debug!("cfg 1: {:?}", cfg);
                    cfg
                },
                None => {
                    error!("No CN provided in configuration");
                    AppConfig {
                        http_port: 80,
                        cn: HashMap::new(),
                    }
                }
            }
        }
        Err(error) => {
            error!("Failed to load configuration from environment: {:?}", error);
            AppConfig {
                http_port: 80,
                cn: HashMap::new(),
            }
        }
    };

    debug!("cfg 2: {:?}", cfg);

    let mut http_server = HttpServer::new(move || {
        let app = App::new()
            .wrap(actix_web::middleware::from_fn(crate::middleware::cors::cors_middleware))

            .app_data(web::Data::new(Arc::new(cfg.clone())))

            .service(web::scope("/session").configure(crate::endpoints::session::config))
            .service(web::scope("/user/sign-up").configure(crate::endpoints::user::registration::config))
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
