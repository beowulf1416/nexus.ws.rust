use tracing::{debug, error, info};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

// use http::header::AUTHORIZATION;
use actix_web::{HttpResponse, Responder, guard, http, web};

use crate::{
    classes::{
        user,
        // tenant,
        // permission
    },
    endpoints::{ApiResponse, default_option_response},
};

use crm_provider::CrmProvider;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("partner/save")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(partner_save_post),
            ),
    )
    .service(
        web::resource("partners/fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(partners_fetch_post),
            ),
    )
    .service(
        web::resource("partners/set/active")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(partners_set_active_post),
            ),
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct PartnerSavePostData {
    tenant_id: uuid::Uuid,
    partner_id: uuid::Uuid,

    business_name: String,
    description: String,

    first_name: String,
    middle_name: String,
    last_name: String,
    prefix: String,
    suffix: String,
    // gender: i16,
}

async fn partner_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<PartnerSavePostData>,
) -> impl Responder {
    info!("partner_save_post");

    let crm_provider = crm_provider_postgres::PostgresCrmProvider::new(&dp);

    let partner = crm_provider::Partner {
        partner_id: params.partner_id.clone(),

        active: true,
        created: chrono::Utc::now(),

        business_name: params.business_name.clone(),
        description: params.description.clone(),

        first_name: params.first_name.clone(),
        middle_name: params.middle_name.clone(),
        last_name: params.last_name.clone(),
        prefix: params.prefix.clone(),
        suffix: params.suffix.clone(),
        // gender: params.gender.clone(),
    };

    match crm_provider.partner_save(&params.tenant_id, &partner).await {
        Err(e) => {
            error!("unable to save partner record: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to save partner record"));
        }
        Ok(_) => {
            return HttpResponse::Ok().json(ApiResponse::ok("successfully saved partner record"));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PartnersFetchPostData {
    tenant_id: uuid::Uuid,
    filter: String,
}

async fn partners_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<PartnersFetchPostData>,
) -> impl Responder {
    info!("partners_fetch_post");

    let crm_provider = crm_provider_postgres::PostgresCrmProvider::new(&dp);

    let filter = format!("%{}%", params.filter);

    match crm_provider
        .partners_fetch(&params.tenant_id, &filter)
        .await
    {
        Err(e) => {
            error!("unable to fetch permissions: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch permissions"));
        }
        Ok(partners) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched permissions",
                Some(json!({
                    "partners": partners
                })),
            ));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PartnersSetActivePostData {
    partner_ids: Vec<uuid::Uuid>,
    active: bool,
}

async fn partners_set_active_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<PartnersSetActivePostData>,
) -> impl Responder {
    info!("partners_set_active_post");

    let crm_provider = crm_provider_postgres::PostgresCrmProvider::new(&dp);
    match crm_provider
        .partners_set_active(&params.partner_ids, params.active)
        .await
    {
        Err(e) => {
            error!("unable to set partner active state: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to save partner record"));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::ok("successfully saved partner active state"));
        }
    }
}
