#![allow(clippy::needless_return)]

use inv_provider::{Item, ItemLocation};
use tracing::{debug, error, info};

use sqlx::{Row, postgres::PgRow, prelude::FromRow};

#[derive(Debug)]
struct ItemRow(pub Item);

impl<'r> FromRow<'r, PgRow> for ItemRow {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(inv_provider::Item {
            item_id: row.get("item_id"),
            active: row.get("active"),
            version: row.get("version"),
            created: row.get("created"),
            updated: row.get("updated"),
            name: row.get("name"),
            description: row.get("description"),
            sku: row.get("sku"),
            upc: row.get("upc"),
            perishable: row.get("perishable"),
            hazardous: row.get("hazardous"),
            flammable: row.get("flammable"),
            esd_sensitive: row.get("esd_sensitive"),
        }));
    }
}

struct ItemLocationRow(pub ItemLocation);

impl<'r> FromRow<'r, PgRow> for ItemLocationRow {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(ItemLocation {
            location_id: row.get("location_id"),
            item_id: row.get("item_id"),
            active: row.get("active"),
            version: row.get("version"),
            created: row.get("created"),
            updated: row.get("updated"),
            batch: row.get("batch"),
            lot: row.get("lot"),
            quantity: row.get("quantity"),
            dimension_id: row.get("dimension_id"),
            uom_id: row.get("uom_id"),
            expiry: row.get("expiry"),
        }));
    }
}

pub struct ItemProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl ItemProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl inv_provider::ItemProvider for ItemProviderPostgres {
    async fn item_save(
        &self,
        tenant_id: &uuid::Uuid,
        item: &inv_provider::Item,
    ) -> Result<(), &'static str> {
        info!("item_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.item_save($1,$2,$3,$4,$5,$6,$7);")
                .bind(tenant_id)
                .bind(item.item_id)
                .bind(item.version)
                .bind(item.name.clone())
                .bind(item.description.clone())
                .bind(item.sku.clone())
                .bind(item.upc.clone())
                .execute(&pool)
                .await
            {
                Err(e) => {
                    error!("Error saving inventory item record: {:?}", e);
                    return Err("Error saving inventory item record");
                }
                Ok(_) => {
                    return Ok(());
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn items_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> Result<Vec<Item>, &'static str> {
        info!("items_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            let filter = format!("%{}%", filter);

            match sqlx::query_as::<_, ItemRow>("select * from mm.items_fetch($1,$2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching inventory items: {:?}", e);
                    return Err("Error fetching inventory items");
                }
                Ok(rows) => {
                    let items = rows.iter().map(|r| r.0.clone()).collect::<Vec<Item>>();
                    return Ok(items);
                }
            }
        }

        return Err("No database pool found");
    }

    async fn location_save(
        &self,
        item_id: &uuid::Uuid,
        location_id: &uuid::Uuid,
    ) -> Result<(), &'static str> {
        info!("location_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.item_location_save($1,$2);")
                .bind(item_id)
                .bind(location_id)
                .execute(&pool)
                .await
            {
                Err(e) => {
                    error!("Error saving location: {:?}", e);
                    return Err("Error saving location");
                }
                Ok(_) => {
                    return Ok(());
                }
            }
        }

        return Err("No database pool found");
    }

    async fn locations_fetch(
        &self,
        item_id: &uuid::Uuid,
    ) -> Result<Vec<ItemLocation>, &'static str> {
        info!("locations_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, ItemLocationRow>("select * from mm.item_locations_fetch($1);")
                .bind(item_id)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error saving location: {:?}", e);
                    return Err("Error saving location");
                }
                Ok(rows) => {
                    let results = rows
                        .iter()
                        .map(|r| r.0.clone())
                        .collect::<Vec<ItemLocation>>();
                    return Ok(results);
                }
            }
        }

        return Err("No database pool found");
    }
}

#[cfg(test)]
mod tests {
    use crate::item::ItemProviderPostgres;
    use crate::location::LocationsProviderPostgres;
    use crate::warehouse::WarehouseProviderPostgres;

    use super::*;

    use inv_provider::{Item, ItemProvider, Location, LocationsProvider, WarehouseProvider};
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_inventory_items() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

        let tenant_id = tp.tenant_fetch_by_name("tenant_01").await.unwrap().id;
        let item_id = uuid::Uuid::new_v4();

        let ipp = ItemProviderPostgres::new(&dp);
        let wpp = WarehouseProviderPostgres::new(&dp);
        let lpp = LocationsProviderPostgres::new(&dp);

        let offset = rand::random::<u16>();

        let item = Item {
            item_id: item_id,
            active: true,
            version: 1,
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            name: format!("item_{}", offset),
            description: format!("item_{}", offset),
            sku: String::from(""),
            upc: String::from(""),
            perishable: false,
            hazardous: false,
            flammable: false,
            esd_sensitive: false,
        };

        if let Err(e) = ipp.item_save(&tenant_id, &item).await {
            error!("unable to create inventory item: {:?}", e);
            assert!(false, "unable to create inventory item");
        }

        if let Err(e) = ipp.items_fetch(&tenant_id, &"%").await {
            error!("unable to fetch inventory items: {:?}", e);
            assert!(false, "unable to fetch inventory items");
        }

        let warehouse_id = wpp
            .warehouses_fetch(&tenant_id, &"Main Warehouse%")
            .await
            .unwrap()[0]
            .warehouse_id;

        let location_id = uuid::Uuid::new_v4();
        let location = Location {
            location_id: location_id,
            version: 0,
            // warehouse_id: warehouse_id,
            name: format!("Main Location {}", offset),
            description: format!("Main Location {}", offset),
            floor: "".to_string(),
            level: "".to_string(),
            section: "".to_string(),
            aisle: "".to_string(),
            row: "".to_string(),
            rack: "".to_string(),
            shelf: "".to_string(),
            bin: "".to_string(),
            pallet: "".to_string(),
        };

        if let Err(e) = lpp
            .location_save(&tenant_id, &warehouse_id, &location)
            .await
        {
            error!("Error saving location: {:?}", e);
            assert!(false, "Error saving location");
        }

        if let Err(e) = ipp.location_save(&item_id, &location_id).await {
            error!("unable to save item location: {:?}", e);
            assert!(false, "unable to save item location");
        }
    }
}
