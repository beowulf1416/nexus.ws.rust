extern crate tracing;

mod extractors;
mod middleware;
mod endpoints;

use tracing::{
    info,
    error,
    debug
};


use serde::Deserialize;

use actix_web::{
    web,
    App,
    HttpServer
};



#[derive(Debug, Deserialize)]
struct Config {
    http_port: Option<u16>,
    cn: Option<Vec<String>>
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // let mut builder = Builder::from_default_env();
    // builder.target(Target::Stdout);
    // builder.init();

    info!("Starting server...");

    match envy::from_env::<Config>() {
        Ok(config) => {
            debug!("Configuration: {:?}", config);
        }
        Err(error) => {
            error!("Failed to load configuration from environment: {:?}", error);
        }
    }

    let mut http_server = HttpServer::new(move || {
        let app = App::new()
            .wrap(actix_web::middleware::from_fn(crate::middleware::cors::cors_middleware))

            .service(web::scope("/session").configure(crate::endpoints::session::config))
            .service(web::scope("/user/sign-up").configure(crate::endpoints::user::registration::config))
        ;

        return app;
    })
    .workers(2)
    ;

    http_server = http_server.bind("localhost:8080")?;

    let server = http_server.run();
    return server.await;
}
