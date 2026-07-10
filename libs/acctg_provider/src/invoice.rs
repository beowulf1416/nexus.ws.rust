#![allow(clippy::needless_return)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceType {
    pub id: i16,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceItem {
    pub item_id: uuid::Uuid,
    pub description: String,
    pub quantity: Decimal,
    // pub uom_id: i32,
    pub unit_price: Decimal,
    pub total: Decimal,
    pub currency_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_id: uuid::Uuid,
    pub invoice_type_id: i16,
    pub invoice_id_seq: i32,

    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub description: String,

    // pub currency_id: i32,
    pub items: Vec<InvoiceItem>,
}

pub trait InvoiceProvider {
    fn invoice_types_fetch(
        &self,
    ) -> impl Future<Output = Result<Vec<InvoiceType>, &'static str>> + Send;

    fn invoices_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<Invoice>, &'static str>> + Send;

    fn invoice_fetch(
        &self,
        invoice_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Invoice, &'static str>> + Send;

    fn invoice_save(
        &self,
        tenant_id: &uuid::Uuid,
        invoice: &Invoice,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    // fn invoice_items_save(
    //     &self,
    //     invoice_id: &uuid::Uuid,
    //     items: &Vec<InvoiceItem>,
    // ) -> impl Future<Output = Result<(), &'static str>> + Send;
}
