extern crate log;

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

    let http_server = HttpServer::new(move || {
        let app = App::new()
            .service(web::scope("/session").configure(crate::endpoints::session::config))
        ;

        return app;
    })
    .workers(2)
    ;

    let server = http_server.run();
    return server.await;
}
