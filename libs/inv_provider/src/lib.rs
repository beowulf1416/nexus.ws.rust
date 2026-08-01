use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod transactions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub item_id: uuid::Uuid,
    pub active: bool,
    pub version: i32,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,

    pub name: String,
    pub description: String,
    pub sku: String,
    pub upc: String,

    pub perishable: bool,
    pub hazardous: bool,
    pub flammable: bool,
    pub esd_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub warehouse_id: uuid::Uuid,
    pub active: bool,
    pub version: i32,
    pub name: String,
    pub description: String,

    pub address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub location_id: uuid::Uuid,
    pub version: i32,
    // pub warehouse_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub floor: String,
    pub level: String,
    pub section: String,
    pub aisle: String,
    pub row: String,
    pub rack: String,
    pub shelf: String,
    pub bin: String,
    pub pallet: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ItemLocation {
    pub item_id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub active: bool,
    pub version: i32,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub batch: String,
    pub lot: String,
    pub quantity: rust_decimal::Decimal,
    pub dimension_id: i32,
    pub uom_id: i32,
    pub expiry: Option<chrono::DateTime<chrono::Utc>>,
}

// pub trait InventoryProvider {
//     // fn item_save(
//     //     &self,
//     //     tenant_id: &uuid::Uuid,
//     //     item: &Item,
//     // ) -> impl Future<Output = Result<(), &'static str>> + Send;

//     fn item_set_active(
//         &self,
//         item_id: &uuid::Uuid,
//         active: &bool,
//     ) -> impl Future<Output = Result<(), &'static str>> + Send;

//     // fn items_fetch(
//     //     &self,
//     //     tenant_id: &uuid::Uuid,
//     //     filter: &str,
//     // ) -> impl Future<Output = Result<Vec<Item>, &'static str>> + Send;

//     fn item_fetch_by_id(
//         &self,
//         item_id: &uuid::Uuid,
//     ) -> impl Future<Output = Result<Item, &'static str>> + Send;
// }

pub trait WarehouseProvider {
    fn warehouse_save(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse: &Warehouse,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn warehouse_set_active(
        &self,
        warehouse_id: &uuid::Uuid,
        active: &bool,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn warehouses_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<Warehouse>, &'static str>> + Send;

    fn fetch_by_name(
        &self,
        tenant_id: &uuid::Uuid,
        name: &str,
    ) -> impl Future<Output = Result<Warehouse, &'static str>> + Send;

    fn fetch_by_id(
        &self,
        warehouse_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Warehouse, &'static str>> + Send;
}

pub trait LocationsProvider {
    fn save(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse_id: &uuid::Uuid,
        location: &Location,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn location_set_active(
        &self,
        location_id: &uuid::Uuid,
        active: &bool,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn fetch(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse_id: &uuid::Uuid,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<Location>, &'static str>> + Send;

    fn fetch_by_name(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse_id: &uuid::Uuid,
        name: &str,
    ) -> impl Future<Output = Result<Location, &'static str>> + Send;
}

pub trait ItemProvider {
    fn item_save(
        &self,
        tenant_id: &uuid::Uuid,
        item: &Item,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn items_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<Item>, &'static str>> + Send;

    fn location_save(
        &self,
        item_id: &uuid::Uuid,
        location_id: &uuid::Uuid,
        version: &i32,
        batch: String,
        lot: String,
        quantity: rust_decimal::Decimal,
        dimension_id: &i32,
        uom_id: &i64,
        expiry: Option<chrono::DateTime<chrono::Utc>>,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn locations_fetch(
        &self,
        item_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Vec<ItemLocation>, &'static str>> + Send;
}
