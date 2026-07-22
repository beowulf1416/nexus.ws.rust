use tracing::{debug, error, info};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

// use http::header::AUTHORIZATION;
use actix_web::{HttpResponse, Responder, guard, http, web};

use crate::{
    classes::user,
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
    )
    .service(
        web::resource("fetch/tree")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(accounts_fetch_tree_post),
            ),
    )
    .service(
        web::resource("fetch/by/type")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(accounts_fetch_by_type_post),
            ),
    )
    .service(
        web::resource("fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(accounts_fetch_post),
            ),
    )
    .service(
        web::resource("account/fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(account_fetch_post),
            ),
    )
    .service(
        web::resource("account/save")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(account_save_post),
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

async fn accounts_fetch_tree_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
) -> impl Responder {
    info!("accounts_fetch_tree_post");

    let app = acctg_provider_postgres::accounts::AccountsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match app.accounts_fetch_tree(&tenant_id).await {
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

#[derive(Debug, Serialize, Deserialize)]
struct AccountFetchByTypePostData {
    type_id: i16,
}

async fn accounts_fetch_by_type_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<AccountFetchByTypePostData>,
) -> impl Responder {
    info!("accounts_fetch_by_type_post");

    let app = acctg_provider_postgres::accounts::AccountsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match app
        .accounts_fetch_by_type(&tenant_id, &params.type_id)
        .await
    {
        Err(e) => {
            error!("unable to fetch accounts by type: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch accounts by type"));
        }
        Ok(accounts) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched accounts by type",
                Some(json!({
                    "accounts": accounts
                })),
            ));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountsFetchPostData {
    account_type_id: i16,
    filter: String,
}

async fn accounts_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<AccountsFetchPostData>,
) -> impl Responder {
    info!("accounts_fetch_post");

    let app = acctg_provider_postgres::accounts::AccountsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match app
        .accounts_fetch(
            &tenant_id,
            &params.account_type_id,
            format!("%{}%", params.filter).as_str(),
        )
        .await
    {
        Err(e) => {
            error!("unable to fetch accounts: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch accounts"));
        }
        Ok(accounts) => {
            // debug!("accounts: {:?}", accounts);
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

#[derive(Debug, Serialize, Deserialize)]
struct AccountFetchPostData {
    account_id: uuid::Uuid,
}

async fn account_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<AccountFetchPostData>,
) -> impl Responder {
    info!("accounts_fetch_post");

    let app = acctg_provider_postgres::accounts::AccountsProviderPostgres::new(&dp);

    match app.account_fetch(&params.account_id).await {
        Err(e) => {
            error!("unable to fetch account: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch account"));
        }
        Ok(account) => {
            // debug!("account: {:?}", account);
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched account",
                Some(json!({
                    "account": account
                })),
            ));
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccountSavePostData {
    account: acctg_provider::accounts::Account,
    parent_account_id: uuid::Uuid,
}

async fn account_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<AccountSavePostData>,
) -> impl Responder {
    info!("account_save_post");
    // debug!("params: {:?}", params);

    let app = acctg_provider_postgres::accounts::AccountsProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match app
        .account_save(&tenant_id, &params.account, &params.parent_account_id)
        .await
    {
        Err(e) => {
            error!("unable to save account: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to save account"));
        }
        Ok(_) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully saved account",
                None,
            ));
        }
    }
}
