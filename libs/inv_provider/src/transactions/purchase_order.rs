use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurchaseOrderItem {
    pub item_id: Uuid,
    pub quantity: rust_decimal::Decimal,
    pub uom_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurchaseOrder {
    pub po_id: Uuid,
    pub active: bool,
    pub version: i32,
    pub description: String,
    pub org_id: Uuid,
    pub partner_id: Uuid,
    pub items: Vec<PurchaseOrderItem>,
}

pub trait PurchaseOrderProvider {
    fn save(
        &self,
        tenant_id: &Uuid,
        purchase_order: &PurchaseOrder,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn fetch_by_id(
        &self,
        po_id: Uuid,
    ) -> impl Future<Output = Result<PurchaseOrder, &'static str>> + Send;
}
