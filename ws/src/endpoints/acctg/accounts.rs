use tracing::{debug, error, info};

use rust_decimal::Decimal;
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

use acctg_provider::accounts::AccountsProvider;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("types/fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(account_types_fetch_post),
            ),
    )
    .service(
        web::resource("categories/fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(account_categories_fetch_post),
            ),
    )
    .service(
        web::resource("fetch/all")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(accounts_fetch_all_post),
            ),
    );
}

async fn account_types_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
) -> impl Responder {
    info!("account_types_fetch_post");

    let app = acctg_provider_postgres::accounts::AccountsProviderPostgres::new(&dp);

    match app.account_types_fetch().await {
        Err(e) => {
            error!("unable to fetch account types: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch account types"));
        }
        Ok(account_types) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched account types",
                Some(json!({
                    "account_types": account_types
                })),
            ));
        }
    }
}

async fn account_categories_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
) -> impl Responder {
    info!("account_categories_fetch_post");

    let app = acctg_provider_postgres::accounts::AccountsProviderPostgres::new(&dp);

    match app.account_categories_fetch().await {
        Err(e) => {
            error!("unable to fetch account categories: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch account categories"));
        }
        Ok(account_categories) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched account categories",
                Some(json!({
                    "account_categories": account_categories
                })),
            ));
        }
    }
}

async fn accounts_fetch_all_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
) -> impl Responder {
    info!("accounts_fetch_all_post");

    let app = acctg_provider_postgres::accounts::AccountsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match app.accounts_fetch_all(&tenant_id).await {
        Err(e) => {
            error!("unable to fetch accounts: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch accounts"));
        }
        Ok(accounts) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched accounts",
                Some(json!({
                    "accounts": accounts
                })),
            ));
        }
    }
}
