#![allow(clippy::needless_return)]

use tracing::{debug, error, info};

use inv_provider::Location;
use sqlx::{Row, postgres::PgRow, prelude::FromRow};

#[derive(Debug, Clone)]
struct LocationData(pub Location);

impl<'r> FromRow<'r, PgRow> for LocationData {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(inv_provider::Location {
            location_id: row.get("location_id"),
            version: row.get("version"),
            name: row.get("name"),
            description: row.get("description"),
            floor: row.get("floor"),
            level: row.get("level"),
            section: row.get("section"),
            aisle: row.get("aisle"),
            row: row.get("row"),
            rack: row.get("rack"),
            shelf: row.get("shelf"),
            bin: row.get("bin"),
            pallet: row.get("pallet"),
        }));
    }
}

pub struct LocationsProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl LocationsProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl inv_provider::LocationsProvider for LocationsProviderPostgres {
    async fn location_save(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse_id: &uuid::Uuid,
        location: &inv_provider::Location,
    ) -> Result<(), &'static str> {
        info!("location_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query(
                "call mm.location_save($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15);",
            )
            .bind(tenant_id)
            .bind(location.location_id)
            .bind(warehouse_id)
            .bind(location.version)
            .bind(location.name.clone())
            .bind(location.description.clone())
            .bind(location.floor.clone())
            .bind(location.level.clone())
            .bind(location.section.clone())
            .bind(location.row.clone())
            .bind(location.rack.clone())
            .bind(location.aisle.clone())
            .bind(location.shelf.clone())
            .bind(location.bin.clone())
            .bind(location.pallet.clone())
            .execute(&pool)
            .await
            {
                Err(e) => {
                    error!("Error saving location record: {:?}", e);
                    return Err("Error saving location record");
                }
                Ok(_) => {
                    return Ok(());
                }
            }
        }

        return Err("No database pool found");
    }

    async fn location_set_active(
        &self,
        location_id: &uuid::Uuid,
        active: &bool,
    ) -> Result<(), &'static str> {
        info!("location_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call mm.location_set_active($1,$2);")
                .bind(location_id)
                .bind(active)
                .execute(&pool)
                .await
            {
                Err(e) => {
                    error!("Error setting location active: {:?}", e);
                    return Err("Error setting location active");
                }
                Ok(_) => {
                    return Ok(());
                }
            }
        }

        return Err("No database pool found");
    }

    async fn fetch(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse_id: &uuid::Uuid,
        filter: &str,
    ) -> Result<Vec<Location>, &'static str> {
        info!("fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            let filter = format!("%{}%", filter);

            match sqlx::query_as::<_, LocationData>("select * from mm.locations_fetch($1, $2, $3);")
                .bind(tenant_id)
                .bind(warehouse_id)
                .bind(&filter)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error setting location active: {:?}", e);
                    return Err("Error setting location active");
                }
                Ok(rows) => {
                    let locations = rows.iter().map(|r| r.0.clone()).collect();
                    return Ok(locations);
                }
            }
        }

        return Err("No database pool found");
    }

    async fn fetch_by_name(
        &self,
        tenant_id: &uuid::Uuid,
        warehouse_id: &uuid::Uuid,
        name: &str,
    ) -> Result<Location, &'static str> {
        info!("fetch_by_name");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, LocationData>(
                "select * from mm.location_fetch_by_name($1, $2, $3);",
            )
            .bind(tenant_id)
            .bind(warehouse_id)
            .bind(name)
            .fetch_one(&pool)
            .await
            {
                Err(e) => {
                    error!("Error setting location active: {:?}", e);
                    return Err("Error setting location active");
                }
                Ok(row) => {
                    return Ok(row.0.clone());
                }
            }
        }

        return Err("No database pool found");
    }
}

#[cfg(test)]
mod tests {
    use crate::warehouse::WarehouseProviderPostgres;

    use super::*;

    use inv_provider::{Location, LocationsProvider, WarehouseProvider};
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_inventory_locations() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let wpp = WarehouseProviderPostgres::new(&dp);
        let lpp = LocationsProviderPostgres::new(&dp);

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
        let tenant_id = tenant.tenant_id();

        let offset = rand::random::<u16>();

        let warehouse_id = uuid::Uuid::new_v4();

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

        if let Err(e) = wpp.warehouse_save(&tenant_id, &wh).await {
            error!("Error saving warehouse: {:?}", e);
            assert!(false, "Error saving warehouse");
        }

        // let warehouse_id = wpp
        //     .warehouse_fetch_by_name(&tenant_id, "location_test_wh")
        //     .await
        //     .unwrap()
        //     .warehouse_id;

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

        if let Err(e) = lpp.fetch(&tenant_id, &warehouse_id, "%").await {
            error!("Error fetching locations: {:?}", e);
            assert!(false, "Error fetching locations");
        }
    }
}
