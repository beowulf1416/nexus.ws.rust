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

use acctg_provider::invoice::{Invoice, InvoiceItem, InvoiceProvider};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("types/fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(invoice_types_fetch_post),
            ),
    )
    .service(
        web::resource("fetch")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(invoices_fetch_post),
            ),
    )
    .service(
        web::resource("fetch/id")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(invoice_fetch_post),
            ),
    )
    .service(
        web::resource("save")
            .route(web::method(http::Method::OPTIONS).to(default_option_response))
            .route(
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(invoice_save_post),
            ),
    );
}

async fn invoice_types_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
) -> impl Responder {
    info!("invoice_types_fetch_post");

    let ipp = acctg_provider_postgres::invoice::InvoiceProviderPostgres::new(&dp);

    match ipp.invoice_types_fetch().await {
        Err(e) => {
            error!("unable to fetch invoice types: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch invoice types"));
        }
        Ok(invoice_types) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched invoice types",
                Some(json!({
                    "invoice_types": invoice_types
                })),
            ));
        }
    }
}

#[derive(Debug, Deserialize)]
struct InvoicesFetchPostData {
    filter: String,
}

async fn invoices_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<InvoicesFetchPostData>,
) -> impl Responder {
    info!("invoices_fetch_post");

    let ipp = acctg_provider_postgres::invoice::InvoiceProviderPostgres::new(&dp);

    let tenant_id = user.tenant().tenant_id();

    match ipp
        .invoices_fetch(&tenant_id, format!("%{}%", &params.filter).as_str())
        .await
    {
        Err(e) => {
            error!("unable to fetch invoices: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch invoices"));
        }
        Ok(invoices) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched invoices",
                Some(json!({
                    "invoices": invoices
                })),
            ));
        }
    }
}

#[derive(Debug, Deserialize)]
struct InvoiceFetchPostData {
    invoice_id: uuid::Uuid,
}

async fn invoice_fetch_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<InvoiceFetchPostData>,
) -> impl Responder {
    info!("invoices_fetch_post");

    let ipp = acctg_provider_postgres::invoice::InvoiceProviderPostgres::new(&dp);

    match ipp.invoice_fetch(&params.invoice_id).await {
        Err(e) => {
            error!("unable to fetch invoice types: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch invoice types"));
        }
        Ok(invoice) => {
            return HttpResponse::Ok().json(ApiResponse::new(
                true,
                "successfully fetched invoice",
                Some(json!({
                    "invoice": invoice
                })),
            ));
        }
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct InvoiceItemSavePostData {
//     item_id: uuid::Uuid,
//     description: String,
//     quantity: Decimal,
//     // uom_id: i32,
//     unit_price: Decimal,
//     total: Decimal,
//     currency_id: i32,
// }

#[derive(Debug, Serialize, Deserialize)]
struct InvoiceSavePostData {
    invoice_id: uuid::Uuid,
    invoice_type_id: i16,
    due_date: Option<chrono::DateTime<chrono::Utc>>,
    description: String,
    // currency_id: i32,
    items: Vec<InvoiceItem>,
}

async fn invoice_save_post(
    dp: web::Data<Arc<database_provider::DatabaseProvider>>,
    user: user::User,
    params: web::Json<InvoiceSavePostData>,
) -> impl Responder {
    info!("invoice_save_post");

    let ipp = acctg_provider_postgres::invoice::InvoiceProviderPostgres::new(&dp);

    let invoice = Invoice {
        invoice_id: params.invoice_id,
        invoice_type_id: params.invoice_type_id,
        invoice_id_seq: 0,
        active: true,
        created_at: chrono::Utc::now(),
        due_date: params.due_date,
        description: params.description.clone(),
        // currency_id: params.currency_id,
        // items: params
        //     .items
        //     .clone()
        //     .into_iter()
        //     .map(|i| InvoiceItem {
        //         item_id: i.item_id,
        //         description: i.description.clone(),
        //         quantity: i.quantity,
        //         // uom_id: i.uom_id,
        //         unit_price: i.unit_price,
        //         total: i.total,
        //         currency_id: i.currency_id,
        //     })
        //     .collect(),
        items: params.items.clone(),
    };

    // match ipp.invoice_save(&user.tenant().tenant_id(), &invoice).await {
    //     Err(e) => {
    //         error!("unable to save invoice: {}", e);
    //         return HttpResponse::InternalServerError()
    //             .json(ApiResponse::error("unable to save invoice"));
    //     }
    //     Ok(_) => {
    //         return HttpResponse::Ok().json(ApiResponse::ok("todo"));
    //     }
    // }

    if let Err(e) = ipp.invoice_save(&user.tenant().tenant_id(), &invoice).await {
        error!("unable to save invoice: {}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::error("unable to save invoice"));
    }

    // if let Err(e) = ipp
    //     .invoice_items_save(&params.invoice_id, &params.items)
    //     .await
    // {
    //     error!("unable to save invoice items: {}", e);
    //     return HttpResponse::InternalServerError()
    //         .json(ApiResponse::error("unable to save invoice items"));
    // }

    return HttpResponse::Ok().json(ApiResponse::ok("successfully saved invoice"));
}
