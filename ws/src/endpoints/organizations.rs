use actix_http::Response;
use tracing::{debug, error, info};

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;

use actix_web::{HttpResponse, Responder, http, web};

use crate::{
    classes::user,
    endpoints::{ApiResponse, default_option_response},
};

use tenants_provider::organizations::OrganizationsProvider;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(organizations_fetch_post)),
    )
    .service(
        web::resource("fetch/tree")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(organizations_fetch_tree_post)),
    )
    .service(
        web::resource("fetch/id")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(organizations_fetch_id_post)),
    )
    .service(
        web::resource("save")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(web::post().to(organization_save_post)),
    );
}

#[derive(Debug, Deserialize)]
struct OrganizationsFetchPost {
    filter: String,
}

async fn organizations_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<OrganizationsFetchPost>,
) -> impl Responder {
    info!("organizations_fetch_post");

    let opp = tenants_provider_postgres::organizations::OrganizationsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();
    let filter = format!("%{}%", params.filter);

    match opp.fetch(&tenant_id, &filter).await {
        Err(e) => {
            error!("unable to fetch organizations: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch organizations"));
        }
        Ok(organizations) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched organizations",
                Some(json!({
                    "organizations": organizations
                })),
            ));
        }
    }
}

async fn organizations_fetch_tree_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
) -> impl Responder {
    info!("organizations_fetch_tree_post");

    let opp = tenants_provider_postgres::organizations::OrganizationsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match opp.fetch_tree(&tenant_id).await {
        Err(e) => {
            error!("unable to fetch organizations: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch organizations"));
        }
        Ok(organizations) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched organizations",
                Some(json!({
                    "organizations": organizations
                })),
            ));
        }
    }
}

#[derive(Debug, Deserialize)]
struct OrganizationsFetchIdPost {
    org_id: uuid::Uuid,
}

async fn organizations_fetch_id_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<OrganizationsFetchIdPost>,
) -> impl Responder {
    info!("organizations_fetch_id_post");

    let opp = tenants_provider_postgres::organizations::OrganizationsProviderPostgres::new(&dp);

    // let tenant_id = user.tenant().tenant_id();

    match opp.fetch_by_id(&params.org_id).await {
        Err(e) => {
            error!("unable to fetch organizations: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch organizations"));
        }
        Ok(organization) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched organizations",
                Some(json!({
                    "organization": organization
                })),
            ));
        }
    }
}

#[derive(Debug, Deserialize)]
struct OrganizationSavePost {
    org_id: uuid::Uuid,
    parent_org_id: uuid::Uuid,
    name: String,
    description: String,
    version: i32,
}

async fn organization_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<OrganizationSavePost>,
) -> impl Responder {
    info!("organization_save_post");

    let opp = tenants_provider_postgres::organizations::OrganizationsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match opp
        .save(
            &tenant_id,
            &params.org_id,
            &params.parent_org_id,
            &params.name,
            &params.description,
            &params.version,
        )
        .await
    {
        Err(e) => {
            error!("unable to save organization: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to save organization"));
        }
        Ok(_) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched organizations",
                None,
            ));
        }
    }
}
