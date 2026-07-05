#![allow(clippy::needless_return)]

use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InvoiceType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceItem {
    pub item_id: uuid::Uuid,
    pub description: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Invoice {
    pub invoice_id: uuid::Uuid,
    pub invoice_type_id: i32,
    pub invoice_id_seq: String,

    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub description: Option<String>,

    pub currency_id: i32,

    pub items: Vec<InvoiceItem>,
}

pub trait InvoiceProvider {
    fn invoice_types_fetch(
        &self,
    ) -> impl Future<Output = Result<Vec<InvoiceType>, &'static str>> + Send;

    fn invoice_save(
        &self,
        tenant_id: &uuid::Uuid,
        invoice: &Invoice,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
}
