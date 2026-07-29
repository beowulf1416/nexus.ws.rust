#![allow(clippy::needless_return)]

use tracing::{debug, error, info};

use sqlx::{Row, postgres::PgRow, prelude::FromRow};

struct WarehouseDataItem(pub inv_provider::Warehouse);

impl<'r> FromRow<'r, PgRow> for WarehouseDataItem {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(inv_provider::Warehouse {
            warehouse_id: row.get("warehouse_id"),
            active: row.get("active"),
            version: row.get("version"),
            name: row.get("name"),
            description: row.get("description"),
            address: inv_provider::Address {
                street: row.get("street"),
                city: row.get("city"),
                state: row.get("state"),
                zip_code: row.get("zip_code"),
                country_id: row.get("country_id"),
            },
        }));
    }
}

pub struct WarehouseProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl WarehouseProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl inv_provider::WarehouseProvider for WarehouseProviderPostgres {
    async fn warehouse_save(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse: &inv_provider::Warehouse,
    ) -> Result<(), &'static str> {
        info!("warehouse_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.warehouse_save($1,$2,$3,$4,$5,$6,$7,$8,$9,$10);")
                .bind(tenant_id)
                .bind(warehouse.warehouse_id)
                .bind(warehouse.name.clone())
                .bind(warehouse.description.clone())
                .bind(warehouse.address.street.clone())
                .bind(warehouse.address.city.clone())
                .bind(warehouse.address.state.clone())
                .bind(warehouse.address.zip_code.clone())
                .bind(warehouse.address.country_id)
                .bind(warehouse.version)
                .execute(&pool)
                .await
            {
                Err(e) => {
                    error!("Error saving warehouse record: {:?}", e);
                    return Err("Error saving warehouse record");
                }
                Ok(_) => {
                    return Ok(());
                }
            }
        }

        return Err("No database pool found");
    }

    async fn warehouse_set_active(
        &self,
        warehouse_id: &uuid::Uuid,
        active: &bool,
    ) -> Result<(), &'static str> {
        info!("warehouse_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.warehouse_set_active($1,$2);")
                .bind(warehouse_id)
                .bind(active)
                .execute(&pool)
                .await
            {
                Err(e) => {
                    error!("Error setting warehouse active status: {:?}", e);
                    return Err("Error setting warehouse active status");
                }
                Ok(_) => {
                    return Ok(());
                }
            }
        }

        return Err("No database pool found");
    }

    async fn warehouses_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> Result<Vec<inv_provider::Warehouse>, &'static str> {
        info!("warehouses_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, WarehouseDataItem>(
                "select * from mm.warehouses_fetch($1,$2);",
            )
            .bind(tenant_id)
            .bind(filter)
            .fetch_all(&pool)
            .await
            {
                Err(e) => {
                    error!("Error fetching warehouse records: {:?}", e);
                    return Err("Error fetching warehouse records");
                }
                Ok(rows) => {
                    let warehouses: Vec<inv_provider::Warehouse> =
                        rows.into_iter().map(|r| r.0).collect();
                    return Ok(warehouses);
                }
            }
        }

        return Err("No database pool found");
    }

    async fn fetch_by_name(
        &self,
        tenant_id: &uuid::Uuid,
        name: &str,
    ) -> Result<inv_provider::Warehouse, &'static str> {
        info!("warehouses_fetch_by_name");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, WarehouseDataItem>(
                "select * from mm.warehouse_fetch_by_name($1,$2);",
            )
            .bind(tenant_id)
            .bind(name)
            .fetch_one(&pool)
            .await
            {
                Err(e) => {
                    error!("Error fetching warehouse record by name: {:?}", e);
                    return Err("Error fetching warehouse record by name");
                }
                Ok(row) => {
                    let warehouse: inv_provider::Warehouse = row.0.clone();
                    return Ok(warehouse);
                }
            }
        }

        return Err("No database pool found");
    }

    async fn fetch_by_id(
        &self,
        warehouse_id: &uuid::Uuid,
    ) -> Result<inv_provider::Warehouse, &'static str> {
        info!("warehouses_fetch_by_name");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, WarehouseDataItem>(
                "select * from mm.warehouse_fetch_by_id($1);",
            )
            .bind(warehouse_id)
            .fetch_one(&pool)
            .await
            {
                Err(e) => {
                    error!("Error fetching warehouse record by id: {:?}", e);
                    return Err("Error fetching warehouse record by id");
                }
                Ok(row) => {
                    let warehouse: inv_provider::Warehouse = row.0.clone();
                    return Ok(warehouse);
                }
            }
        }

        return Err("No database pool found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use inv_provider::WarehouseProvider;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_inventory_warehouses() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let provider = WarehouseProviderPostgres::new(&dp);

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
        let tenant_id = tenant.tenant_id();

        let offset = rand::random::<u16>();

        let wh = inv_provider::Warehouse {
            warehouse_id: uuid::Uuid::new_v4(),
            active: true,
            version: 0,
            name: format!("Main Warehouse {}", offset),
            description: format!("Main Warehouse {}", offset),
            address: inv_provider::Address {
                street: format!("street_{}", offset),
                city: format!("city_{}", offset),
                state: format!("state_{}", offset),
                zip_code: format!("zip_{}", offset),
                country_id: 840, // USA
            },
        };

        if let Err(e) = provider.warehouse_save(&tenant_id, &wh).await {
            error!("Error saving warehouse: {:?}", e);
            assert!(false, "Error saving warehouse");
        }

        if let Err(e) = provider.warehouse_set_active(&wh.warehouse_id, &true).await {
            error!("Error setting warehouse active: {:?}", e);
            assert!(false, "Error setting warehouse active");
        }

        if let Err(e) = provider.fetch_by_id(&wh.warehouse_id).await {
            error!("Error fetching warehouse by id: {:?}", e);
            assert!(false, "Error fetching warehouse by id");
        }

        if let Err(e) = provider
            .fetch_by_name(&tenant_id, &format!("Main Warehouse {}", offset))
            .await
        {
            error!("Error fetching warehouse by name: {:?}", e);
            assert!(false, "Error fetching warehouse by name");
        }
    }
}
