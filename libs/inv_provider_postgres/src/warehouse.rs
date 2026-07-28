#![allow(clippy::needless_return)]

use tracing::{debug, error, info};

use sqlx::{Row, postgres::PgRow, prelude::FromRow};

struct WarehouseDataItem(pub inv_provider::Warehouse);

impl<'r> FromRow<'r, PgRow> for WarehouseDataItem {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(inv_provider::Warehouse {
            id: row.get("warehouse_id"),
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

pub struct PostgresWarehouseProvider {
    dp: database_provider::DatabaseProvider,
}

impl PostgresWarehouseProvider {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl inv_provider::WarehouseProvider for PostgresWarehouseProvider {
    async fn warehouse_save(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse: &inv_provider::Warehouse,
    ) -> Result<(), &'static str> {
        info!("warehouse_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.warehouse_save($1,$2,$3,$4,$5,$6,$7,$8,$9,$10);")
                .bind(tenant_id)
                .bind(warehouse.id)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use inv_provider::WarehouseProvider;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_inventory() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let provider = PostgresWarehouseProvider::new(&dp);

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let tenant = tp.tenant_fetch_by_name("default").await.unwrap();
        let tenant_id = tenant.tenant_id();

        let wh = inv_provider::Warehouse {
            id: uuid::Uuid::new_v4(),
            active: true,
            version: 0,
            name: "Main Warehouse".to_string(),
            description: "The primary warehouse".to_string(),
            address: inv_provider::Address {
                street: "123 Main St".to_string(),
                city: "Metropolis".to_string(),
                state: "NY".to_string(),
                zip_code: "12345".to_string(),
                country_id: 840, // USA
            },
        };

        if let Err(e) = provider.warehouse_save(&tenant_id, &wh).await {
            error!("Error saving warehouse: {:?}", e);
            assert!(false, "Error saving warehouse");
        }
    }
}
