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
