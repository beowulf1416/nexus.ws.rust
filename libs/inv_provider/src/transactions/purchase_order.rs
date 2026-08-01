use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct PurchaseOrder {
    pub po_id: Uuid,
    pub active: bool,
    pub version: i32,
    pub description: String,
    pub org_id: Uuid,
    pub partner_id: Uuid,
}

pub trait PurchaseOrderProvider {
    fn save(
        &self,
        tenant_id: &Uuid,
        purchase_order: PurchaseOrder,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
}
