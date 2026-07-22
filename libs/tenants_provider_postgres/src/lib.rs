#![allow(clippy::needless_return)]

pub mod organizations;

use tracing::{debug, error, info};

use sqlx::{Row, postgres::PgRow, prelude::FromRow, types::chrono};
use std::vec::Vec;
use uuid::Uuid;

struct TenantItem(pub tenants_provider::Tenant);

impl<'r> FromRow<'r, PgRow> for TenantItem {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(tenants_provider::Tenant {
            id: row.get("tenant_id"),
            active: row.get("active"),
            version: row.get("version"),
            created: row.get("created"),
            updated: row.get("updated"),
            name: row.get("name"),
            description: row.get("description"),
        }));
    }
}

struct PermissionItem(pub tenants_provider::Permission);

impl<'r> FromRow<'r, PgRow> for PermissionItem {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(tenants_provider::Permission {
            id: row.get("permission_id"),
            name: row.get("name"),
        }));
    }
}

pub struct PostgresTenantsProvider {
    dp: database_provider::DatabaseProvider,
}

impl PostgresTenantsProvider {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl tenants_provider::TenantsProvider for PostgresTenantsProvider {
    async fn tenants_fetch_by_id(
        &self,
        tenant_id: &uuid::Uuid,
    ) -> Result<tenants_provider::Tenant, &'static str> {
        info!("tenants_fetch_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, TenantItem>("select * from tenants.tenants_fetch_by_id($1);")
                .bind(tenant_id)
                .fetch_one(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching tenant record: {:?}", e);
                    return Err("Error fetching tenant record");
                }
                Ok(r) => {
                    return Ok(r.0);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn tenant_fetch_by_name(
        &self,
        name: &str,
    ) -> Result<tenants_provider::Tenant, &'static str> {
        info!("tenant_fetch_by_name");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, TenantItem>("select * from tenants.tenant_fetch_by_name($1);")
                .bind(name)
                .fetch_one(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching tenant record by name: {:?}", e);
                    return Err("Error fetching tenant record by name");
                }
                Ok(r) => {
                    return Ok(r.0);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn tenant_save(
        &self,
        tenant_id: &uuid::Uuid,
        name: &str,
        description: &str,
        version: &i32,
    ) -> Result<(), &'static str> {
        info!("tenant_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenant_save($1,$2,$3,$4);")
                .bind(tenant_id)
                .bind(name)
                .bind(description)
                .bind(version)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error saving tenant record: {:?}", e);
                    return Err("Error saving tenant record");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn tenant_set_active(
        &self,
        tenant_id: &uuid::Uuid,
        active: &bool,
    ) -> Result<(), &'static str> {
        info!("tenant_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenant_set_active($1,$2);")
                .bind(tenant_id)
                .bind(active)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error setting tenant active state: {:?}", e);
                    return Err("Error setting tenant active state");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn tenants_set_active(
        &self,
        tenant_ids: &Vec<uuid::Uuid>,
        active: &bool,
    ) -> Result<(), &'static str> {
        info!("tenants_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenants_set_active($1,$2);")
                .bind(tenant_ids)
                .bind(active)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error setting tenants active state: {:?}", e);
                    return Err("Error setting tenants active state");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn tenants_fetch(
        &self,
        filter: &str,
    ) -> Result<Vec<tenants_provider::Tenant>, &'static str> {
        info!("tenants_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, TenantItem>("select * from tenants.tenants_fetch($1);")
                .bind(filter)
                .fetch_all(&pool)
                .await
            {
                Ok(rows) => {
                    let tenants = rows.iter().map(|r| r.0.clone()).collect();
                    return Ok(tenants);
                }
                Err(e) => {
                    error!("Error fetching tenant records: {:?}", e);
                    return Err("Error fetching tenant records");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn tenant_user_tenants_fetch(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<tenants_provider::Tenant>, &'static str> {
        info!("tenant_user_fetch_tenants");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, TenantItem>(
                "select * from tenants.tenant_user_fetch_tenants($1);",
            )
            .bind(user_id)
            .fetch_all(&pool)
            .await
            {
                Ok(rows) => {
                    let tenants: Vec<tenants_provider::Tenant> =
                        rows.iter().map(|r| r.0.clone()).collect();
                    return Ok(tenants);
                }
                Err(e) => {
                    error!("Error fetching tenant records: {:?}", e);
                    return Err("Error fetching tenant records");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn tenant_user_permissions_fetch(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid,
    ) -> Result<Vec<tenants_provider::Permission>, &'static str> {
        info!("tenant_user_permissions_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, PermissionItem>(
                "select * from tenants.tenant_user_fetch_permissions($1, $2);",
            )
            .bind(user_id)
            .bind(tenant_id)
            .fetch_all(&pool)
            .await
            {
                Ok(rows) => {
                    let permissions: Vec<tenants_provider::Permission> =
                        rows.iter().map(|r| r.0.clone()).collect();
                    return Ok(permissions);
                }
                Err(e) => {
                    error!("Error fetching permissions: {:?}", e);
                    return Err("Error fetching permissions");
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
    use super::*;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_tenants() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tenant_id = uuid::Uuid::new_v4();
        let name = format!("test_{}", rand::random::<u16>());
        let description = "tenants_provider_postgres_test";

        let tp = PostgresTenantsProvider::new(&dp);

        if let Err(e) = tp.tenant_save(&tenant_id, &name, &description, &0).await {
            error!(e);
            assert!(false, "unable to save tenant record");
        }

        if let Err(e) = tp.tenant_fetch_by_name(&name).await {
            error!(e);
            assert!(false, "unable to fetch tenant record by name");
        }

        if let Err(e) = tp.tenant_set_active(&tenant_id, &true).await {
            error!(e);
            assert!(false, "unable to set tenant active state");
        }

        let tenant_ids = vec![tenant_id];
        if let Err(e) = tp.tenants_set_active(&tenant_ids, &true).await {
            error!(e);
            assert!(false, "unable to set tenant active state");
        }

        if let Err(e) = tp.tenants_fetch_by_id(&tenant_id).await {
            error!(e);
            assert!(false, "unable to fetch tenant");
        }

        if let Err(e) = tp.tenants_fetch("%test%").await {
            error!(e);
            assert!(false, "unable to fetch tenants");
        }
    }
}
