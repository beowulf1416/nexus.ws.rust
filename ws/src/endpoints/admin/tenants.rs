use serde::Deserialize;
use tracing::{
    info,
    debug,
    error
};
use std::sync::Arc;
use serde_json::json;
use actix_web::{
    http, 
    web, 
    HttpResponse, 
    Responder
};


use crate::endpoints::{
    ApiResponse,
    default_option_response
};

use admin_tenants::tenants::AdminTenantsProvider;





pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("fetch/id")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(admin_tenants_fetch_id))
        )
        .service(
            web::resource("save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(admin_tenants_save))
        )
        .service(
            web::resource("fetch")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().to(admin_tenants_fetch))
        )
    ;
}



#[derive(Debug, Deserialize)]
struct AdminTenantFetchById {
    tenant_id: uuid::Uuid
}

async fn admin_tenants_fetch_id(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<AdminTenantFetchById>
) -> impl Responder {
    info!("admin_tenants_fetch_id");

    let atp = admin_tenants_postgres::tenants::PostgresAdminTenantsProvider::new(&dp);

    match atp.tenants_fetch_by_id(&params.tenant_id).await {
        Err(e) => {
            error!("unable to fetch tenant by id: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch tenant by id"));
        }
        Ok(tenant) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    &"successfully retrieved tenant by id",
                    Some(json!({
                        "tenant": tenant
                    }))
                ));
        }
    }
}



#[derive(Debug, Deserialize)]
struct AdminTenantSavePost {
    tenant_id: uuid::Uuid,
    name: String,
    description: String
}


async fn admin_tenants_save(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<AdminTenantSavePost>
) -> impl Responder {
    info!("admin_tenants_save");

    let atp = admin_tenants_postgres::tenants::PostgresAdminTenantsProvider::new(&dp);

    if let Err(e) = atp.tenant_save(
        &params.tenant_id, 
        &params.name, 
        &params.description
    ).await {
        error!("unable to save tenant: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("unable to save tenant"));
    }

    return HttpResponse::Ok()
        .json(ApiResponse::ok("success"))
        ;
}




#[derive(Debug, Deserialize)]
struct AdminTenantsFetchPost {
    filter: String
}

async fn admin_tenants_fetch(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    params: web::Json<AdminTenantsFetchPost>
) -> impl Responder {
    info!("admin_tenants_fetch");

    let atp = admin_tenants_postgres::tenants::PostgresAdminTenantsProvider::new(&dp);

    let filter = format!("%{}%", params.filter);

    match atp.tenants_fetch(
        filter.as_str()
    ).await {
        Err(e) => {
            error!("unable to fetch tenant records: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch tenant records"));
        }
        Ok(tenants) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved tenant records",
                    Some(json!({
                        "tenants": tenants
                    }))
                ));
        }
    }
}