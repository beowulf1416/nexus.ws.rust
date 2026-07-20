#![allow(clippy::needless_return)]

use organizations_provider::{OrganizationTreeItem, OrganizationsProvider};
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, prelude::FromRow};
use std::collections::HashMap;
use tracing::{debug, error, info};

pub struct OrganizationTreeItemImpl(pub OrganizationTreeItem);

impl<'r> FromRow<'r, PgRow> for OrganizationTreeItemImpl {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        return Ok(Self(OrganizationTreeItem {
            org_id: row.get("org_id"),
            active: row.get("active"),
            created: row.get("created"),
            name: row.get("name"),
            description: row.get("description"),
            parent_org_id: row.get("parent_org_id"),
            level: row.get("level"),
            path: row.get("path"),
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

impl OrganizationsProvider for OrganizationsProviderPostgres {
    async fn organizations_save(
        &self,
        tenant_id: &uuid::Uuid,
        organization: &organizations_provider::Organization,
    ) -> Result<(), &'static str> {
        info!("organizations_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call organizations.organization_save($1, $2, $3, $4, $5);")
                .bind(&tenant_id)
                .bind(&organization.org_id)
                .bind(&organization.name)
                .bind(&organization.description)
                .bind(&organization.parent_org_id)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error saving organization: {:?}", e);
                    return Err("Error saving organization");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn organizations_fetch_tree(
        &self,
        tenant_id: &uuid::Uuid,
    ) -> Result<Vec<OrganizationTreeItem>, &'static str> {
        info!("organizations_fetch_tree");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, OrganizationTreeItemImpl>(
                "select * from organizations.organizations_fetch_tree($1);",
            )
            .bind(&tenant_id)
            .fetch_all(&pool)
            .await
            {
                Ok(rows) => {
                    let items: Vec<OrganizationTreeItem> =
                        rows.iter().map(|r| r.0.clone()).collect();
                    return Ok(items);
                }
                Err(e) => {
                    error!("Error fetching organizations: {:?}", e);
                    return Err("Error fetching organizations");
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

    use organizations_provider::OrganizationsProvider;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_organizations() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let opp = OrganizationsProviderPostgres::new(&dp);

        let name = format!("test_{}", rand::random::<u16>());

        let tenant_id = tp
            .tenant_fetch_by_name("tenant_01")
            .await
            .unwrap()
            .tenant_id();

        let org_01 = organizations_provider::Organization {
            org_id: uuid::Uuid::new_v4(),
            name: name.clone(),
            description: name.clone(),
            parent_org_id: tenant_id,
        };

        if let Err(e) = opp.organizations_save(&tenant_id, &org_01).await {
            error!(e);
            assert!(false, "unable to save organizations");
        }

        if let Err(e) = opp.organizations_fetch_tree(&tenant_id).await {
            error!(e);
            assert!(false, "unable to fetch tree of organizations");
        }
    }
}
