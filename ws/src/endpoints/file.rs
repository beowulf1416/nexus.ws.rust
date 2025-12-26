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


use file_provider::FileProvider;
use crate::{
    endpoints::{
        ApiResponse,
        default_option_response
    },
    classes::user
};
use crate::middleware::permissions::Permission;




pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("upload")
                .wrap(Permission::new("files.upload"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(file_upload_post))
        )
        .service(
            web::resource("folder/create")
                .wrap(Permission::new("files.folders.create"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(folder_create_post))
        )
        .service(
            web::resource("folder/list/folders")
                .wrap(Permission::new("files.folders.list.folders"))
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(folder_list_folders_post))
        )
    ;
}



async fn file_upload_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    mut payload: Multipart
) -> impl Responder {
    info!("file_upload_post");

    let mut folder_id: uuid::Uuid = uuid::Uuid::nil();
    let mut file_id: uuid::Uuid = uuid::Uuid::nil();
    let mut file_name: String = String::new();
    let mut file_uploaded = false;

    while let Some(p) = payload.next().await {
        let mut field = p.unwrap();
        let cd = field.content_disposition().unwrap();
        let field_name = cd.get_name().unwrap();
        match field_name {
            "file" => {
                file_name = cd.get_filename().map(String::from).unwrap();
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

                file_uploaded = true;
            },
            "folder_id" => {
                let mut data = Vec::new();
                while let Some(bytes) = field.next().await {
                    match bytes {
                        Err(e) => {
                            error!("error reading folder_id bytes: {:?}", e);
                            return HttpResponse::InternalServerError()
                                .json(ApiResponse::error("Error reading folder_id"));
                        }
                        Ok(b) => {
                            data.extend_from_slice(&b);
                        }
                    }
                }

                if let Ok(id_str) = String::from_utf8(data.clone()) {
                    if let Ok(id) = uuid::Uuid::parse_str(&id_str) {
                        folder_id = id;
                    } else {
                        return HttpResponse::BadRequest()
                            .json(ApiResponse::error("Invalid folder_id"));
                    }
                }
            },
            "file_id" => {
                let mut data = Vec::new();
                while let Some(bytes) = field.next().await {
                    match bytes {
                        Err(e) => {
                            error!("error reading file_id bytes: {:?}", e);
                            return HttpResponse::InternalServerError()
                                .json(ApiResponse::error("Error reading file_id"));
                        }
                        Ok(b) => {
                            data.extend_from_slice(&b);
                        }
                    }
                }

                if let Ok(id_str) = String::from_utf8(data.clone()) {
                    if let Ok(id) = uuid::Uuid::parse_str(&id_str) {
                        file_id = id;
                    } else {
                        return HttpResponse::BadRequest()
                            .json(ApiResponse::error("Invalid file_id"));
                    }
                }
            }
            _ => {
                debug!("unhandled field: {}", field_name);
            }
        }
    }

    if file_uploaded {
        let fp = file_provider_postgres::PostgresFileProvider::new(&dp);
        if let Err(e) = fp.file_add(
            &user.tenant().tenant_id(), 
            &folder_id, 
            &file_provider::File::new(
                file_id,
                file_name.clone()
            )
        ).await {
            error!("error adding file to provider: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("Error saving file"));
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("File uploaded successfully"));
}



#[derive(Debug, Deserialize)]
struct FolderCreatePost {
    folder_id: uuid::Uuid,
    name: String
}

async fn folder_create_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<FolderCreatePost>
) -> impl Responder {
    info!("folder_create_post");

    let fp = file_provider_postgres::PostgresFileProvider::new(&dp);
    match fp.folder_add(
        &user.tenant().tenant_id(),
        &file_provider::Folder::new(
            params.folder_id,
            params.name.clone()
        )
    ).await {
        Err(e) => {
            error!("error creating folder: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("Error creating folder"));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::ok("Folder created successfully"));
        }
    }
}


#[derive(Debug, Deserialize)]
struct FolderListFoldersPost {
    folder_id: uuid::Uuid
}

async fn folder_list_folders_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<FolderListFoldersPost>
) -> impl Responder {
    info!("folder_list_folders_post");

    let fp = file_provider_postgres::PostgresFileProvider::new(&dp);

    let f1 = fp.folder_list_folders(&params.folder_id);
    let f2 = fp.folder_list_files(&params.folder_id);

    match futures::try_join!(f1, f2) {
        Err(e) => {
            error!("unable to fetch files and folders: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch files and folders"));
        }
        Ok((folders, files)) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully fetched files and folders",
                    Some(json!({
                        "files": files,
                        "folders": folders
                    }))
                ));
        }
    }
}