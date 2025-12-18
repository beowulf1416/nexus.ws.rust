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

    while let Some(p) = payload.next().await {
        let mut field = p.unwrap();
        let cd = field.content_disposition().unwrap();
        let field_name = cd.get_name().unwrap();
        match field_name {
            "file" => {
                let file_name = cd.get_filename().map(String::from).unwrap();
                // debug!("Receiving file: {}", file_name);

                let mut file = match tokio::fs::File::create(format!("/var/tmp/{}", file_name)).await {
                    Err(e) => {
                        error!("error creating file: {:?}", e);
                        return HttpResponse::InternalServerError()
                            .json(ApiResponse::error("Error creating file"));
                    }
                    Ok(f) => f
                };

                while let Some(next_chunk) = field.next().await {
                    let chunk = match next_chunk {
                        Err(e) => {
                            error!("error reading chunk: {:?}", e);
                            return HttpResponse::InternalServerError()
                                .json(ApiResponse::error("Error reading file chunk"));
                        }
                        Ok(c) => {
                            debug!("read chunk of size: {}", c.len());
                            c
                        }
                    };

                    if let Err(e) = file.write_all(&chunk).await {
                        error!("error writing chunk to file: {:?}", e);
                        return HttpResponse::InternalServerError()
                            .json(ApiResponse::error("Error writing file chunk"));
                    }
                }
            },
            _ => {
                debug!("unhandled field: {}", field_name);
            }
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("File uploaded successfully"));
}