#![allow(clippy::needless_return)]

use tracing::{debug, error, info};

use sqlx::{Row, postgres::PgRow, prelude::FromRow};

struct OrganizationDataItem(pub tenants_provider::organizations::OrganizationData);

impl<'r> FromRow<'r, PgRow> for OrganizationDataItem {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(tenants_provider::organizations::OrganizationData {
            organization_id: row.get("org_id"),
            name: row.get("name"),
            description: row.get("description"),
        }));
    }
}

pub struct OrganizationsProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl OrganizationsProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl tenants_provider::organizations::OrganizationsProvider for OrganizationsProviderPostgres {
    async fn save(
        &self,
        tenant_id: &uuid::Uuid,
        org_id: &uuid::Uuid,
        parent_org_id: &uuid::Uuid,
        name: &str,
        description: &str,
        version: &i32,
    ) -> Result<(), &'static str> {
        info!("save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call organizations.organization_save($1,$2,$3,$4,$5,$6);")
                .bind(tenant_id)
                .bind(org_id)
                .bind(parent_org_id)
                .bind(name)
                .bind(description)
                .bind(version)
                .execute(&pool)
                .await
            {
                Err(e) => {
                    error!("Error saving organization: {:?}", e);
                    return Err("Error saving organization");
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

    async fn fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> Result<Vec<tenants_provider::organizations::OrganizationData>, &'static str> {
        info!("fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, OrganizationDataItem>(
                "select * from organizations.organizations_fetch($1, $2);",
            )
            .bind(tenant_id)
            .bind(filter)
            .fetch_all(&pool)
            .await
            {
                Err(e) => {
                    error!("Error fetching organizations: {:?}", e);
                    return Err("Error fetching organizations");
                }
                Ok(rows) => {
                    let organizations = rows.iter().map(|r| r.0.clone()).collect();
                    return Ok(organizations);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn fetch_by_id(
        &self,
        org_id: &uuid::Uuid,
    ) -> Result<tenants_provider::organizations::OrganizationData, &'static str> {
        info!("fetch_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, OrganizationDataItem>(
                "select * from organizations.organization_fetch_by_id($1);",
            )
            .bind(org_id)
            .fetch_one(&pool)
            .await
            {
                Err(e) => {
                    error!("Error fetching organizations: {:?}", e);
                    return Err("Error fetching organizations");
                }
                Ok(row) => {
                    return Ok(row.0.clone());
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
    use crate::PostgresTenantsProvider;
    use tenants_provider::{TenantsProvider, organizations::OrganizationsProvider};

    #[actix_web::test]
    async fn test_tenants() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = PostgresTenantsProvider::new(&dp);
        let opp = OrganizationsProviderPostgres::new(&dp);

        let tenant_id = tp
            .tenant_fetch_by_name("tenant_01")
            .await
            .unwrap()
            .tenant_id();

        let org_id = uuid::Uuid::new_v4();
        let parent_org_id = opp.fetch_by_id(&tenant_id).await.unwrap().organization_id;
        let name = format!("test_{}", rand::random::<u16>());
        let description = "tenants_provider_postgres_test";
        let version = 0;

        if let Err(e) = opp
            .save(
                &tenant_id,
                &org_id,
                &parent_org_id,
                &name,
                &description,
                &version,
            )
            .await
        {
            error!(e);
            assert!(false, "unable to save organization");
        }

        if let Err(e) = opp.fetch(&tenant_id, &"%").await {
            error!(e);
            assert!(false, "unable to fetch organizations");
        }
    }
}
