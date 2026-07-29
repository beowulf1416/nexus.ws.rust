#![allow(clippy::needless_return)]

use tracing::{debug, error, info};

use sqlx::{Row, postgres::PgRow, prelude::FromRow};

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
}

#[cfg(test)]
mod tests {
    use crate::item::ItemProviderPostgres;

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

        let ipp = ItemProviderPostgres::new(&dp);

        let offset = rand::random::<u16>();

        let item = Item {
            item_id: uuid::Uuid::new_v4(),
            active: true,
            version: 1,
            // created: chrono::Utc::now(),
            // updated: chrono::Utc::now(),
            name: format!("item_{}", offset),
            description: format!("item_{}", offset),
            sku: String::from(""),
            upc: String::from(""),
        };

        if let Err(e) = ipp.item_save(&tenant_id, &item).await {
            error!("unable to create inventory item: {:?}", e);
            assert!(false, "unable to create inventory item");
        }
    }
}
