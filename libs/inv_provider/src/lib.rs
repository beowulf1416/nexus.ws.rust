pub struct Item {
    pub id: uuid::Uuid,
    pub active: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: String,
    pub sku: String,
    pub upc: String 
}



pub trait InventoryProvider {

    fn item_save(
        &self,
        tenant_id: &uuid::Uuid,
        item: &Item
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn item_set_active(
        &self,
        item_id: &uuid::Uuid,
        active: &bool
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn items_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str
    ) -> impl Future<Output = Result<Vec<Item>, &'static str>> + Send;

    fn item_fetch_by_id(
        &self,
        item_id: &uuid::Uuid
    ) -> impl Future<Output = Result<Item, &'static str>> + Send;
}