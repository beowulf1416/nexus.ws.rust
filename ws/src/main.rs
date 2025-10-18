extern crate log;

mod extractors;
mod middleware;
mod endpoints;

use log::{
    info,
    debug
};

use actix_web::{
    web,
    App,
    HttpServer
};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

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
