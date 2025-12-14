#![allow(clippy::needless_return)]

use tracing::{
    info,
    error,
    debug
};

use sqlx::Row;


pub struct PostgresInventoryProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresInventoryProvider {
    pub fn new(
        dp: &database_provider::DatabaseProvider
     ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}


impl inv_provider::InventoryProvider for PostgresInventoryProvider {

    async fn item_save(
        &self,
        tenant_id: &uuid::Uuid,
        item: &inv_provider::Item
    ) -> Result<(), &'static str> {
        info!("item_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.item_save($1,$2,$3,$4,$5,$6);")
                .bind(tenant_id)
                .bind(item.id)
                .bind(item.name.clone())
                .bind(item.description.clone())
                .bind(item.sku.clone())
                .bind(item.upc.clone())
                .execute(&pool)
                .await {
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

    async fn item_set_active(
        &self,
        item_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), &'static str> {
        info!("item_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.item_set_active($1,$2);")
                .bind(item_id)
                .bind(active)
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error setting inventory item active state: {:?}", e);
                        return Err("Error setting inventory item active state");
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
        filter: &str
    ) -> Result<Vec<inv_provider::Item>, &'static str> {
        info!("items_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.items_fetch($1,$2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await {
                    Err(e) => {
                        error!("Error fetching inventory item records: {:?}", e);
                        return Err("Error fetching inventory item records");
                    }
                    Ok(rows) => {
                        let items = rows.iter().map(|r| {
                            let id: uuid::Uuid = r.get("item_id");
                            let active: bool = r.get("active");
                            let created: chrono::DateTime<chrono::Utc> = r.get("created");
                            let name: String = r.get("name");
                            let description: String = r.get("description");
                            let sku: String = r.get("sku");
                            let upc: String = r.get("upc");

                            return inv_provider::Item {
                                id,
                                active,
                                created,
                                name,
                                description,
                                sku,
                                upc
                            };
                        }).collect();
                        return Ok(items);
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn item_fetch_by_id(
        &self,
        item_id: &uuid::Uuid
    ) -> Result<inv_provider::Item, &'static str> {
        info!("item_fetch_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.items_fetch($1,$2);")
                .bind(item_id)
                .fetch_one(&pool)
                .await {
                    Err(e) => {
                        error!("Error fetching inventory item record: {:?}", e);
                        return Err("Error fetching inventory item record");
                    }
                    Ok(r) => {
                        let id: uuid::Uuid = r.get("item_id");
                        let active: bool = r.get("active");
                        let created: chrono::DateTime<chrono::Utc> = r.get("created");
                        let name: String = r.get("name");
                        let description: String = r.get("description");
                        let sku: String = r.get("sku");
                        let upc: String = r.get("upc");

                        return Ok(inv_provider::Item {
                            id,
                            active,
                            created,
                            name,
                            description,
                            sku,
                            upc
                        });
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }
    // Other methods would be implemented similarly...
}


#[cfg(test)]
mod tests {
    use super::*;

    use inv_provider::InventoryProvider;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_inventory() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let provider = PostgresInventoryProvider::new(&dp);

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let tenant = tp.tenant_fetch_by_name("default").await.unwrap();
        let tenant_id = tenant.tenant_id();

        let new_item = inv_provider::Item {
            id: uuid::Uuid::new_v4(),
            active: true,
            created: chrono::Utc::now(),
            name: "Test Item".to_string(),
            description: "This is a test item".to_string(),
            sku: "TESTSKU".to_string(),
            upc: "123456789012".to_string()
        };
        
        if let Err(e) = provider.item_save(&tenant_id, &new_item).await {
            error!("unable to create inventory item: {:?}", e);
            assert!(false, "unable to create inventory item");
        }

        if let Err(e) = provider.item_set_active(&new_item.id, &true).await {
            error!("unable to create inventory item: {:?}", e);
            assert!(false, "unable to create inventory item");
        }
    }
}
