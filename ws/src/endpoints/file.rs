use tracing::{
    info,
    error,
    debug
};

use std::sync::Arc;
use std::vec::Vec;

use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use futures::StreamExt;

use actix_web::{
    http, 
    web::{
        self, 
        Bytes
    }, 
    dev::Payload,
    HttpResponse, 
    Responder
};
use actix_multipart::{
    Field,
    Multipart
};

use tokio::{io::AsyncWriteExt};



use crate::endpoints::{
    ApiResponse,
    default_option_response
};




pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("upload")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(file_upload_post))
        )
    ;
}



async fn file_upload_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    mut payload: Multipart
) -> impl Responder {
    info!("file_upload_post");

    let mut file: tokio::fs::File = match tokio::fs::File::create("/var/tmp/upload_test_file.txt").await {
        Err(e) => {
            error!("error creating file: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("Error creating file"));
        }
        Ok(f) => f
    };

    while let Some(p) = payload.next().await {
        let mut field = p.unwrap();
        let cd = field.content_disposition().unwrap();
        let field_name = cd.get_name().unwrap();
        // let file_name = cd.get_filename().map(|s| s.to_string());

        debug!("Processing field: {}", field_name);

        match field_name {
            "file" => {
                let file_name = cd.get_filename().map(String::from);
                let file_content = Some(field.map(|chunk| chunk.unwrap()).collect::<Vec<Bytes>>().await);
                debug!("Received file field: filename = {:?}", file_name);
                debug!("Received file field: content = {:?}", file_content);
            },
            _ => {
                debug!("unhandled field: {}", field_name);
            }
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("File uploaded successfully"));
}