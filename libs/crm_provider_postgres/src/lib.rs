#![allow(clippy::needless_return)]

use tracing::{
    error,
    // debug
    info,
};

use sqlx::{Row, postgres::PgRow, prelude::FromRow};

struct Partnerdata(pub crm_provider::Partner);

impl<'r> FromRow<'r, PgRow> for Partnerdata {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(crm_provider::Partner {
            partner_id: row.get("partner_id"),
            active: row.get("active"),
            created: row.get("created_ts"),
            business_name: row.get("business_name"),
            description: row.get("description"),
            first_name: row.get("first_name"),
            middle_name: row.get("middle_name"),
            last_name: row.get("last_name"),
            prefix: row.get("prefix"),
            suffix: row.get("suffix"),
        }));
    }
}

pub struct PostgresCrmProvider {
    dp: database_provider::DatabaseProvider,
}

impl PostgresCrmProvider {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl crm_provider::CrmProvider for PostgresCrmProvider {
    async fn partner_save(
        &self,
        tenant_id: &uuid::Uuid,
        partner: &crm_provider::Partner,
    ) -> Result<(), &'static str> {
        info!("partner_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call crm.partner_save($1, $2, $3, $4, $5, $6, $7, $8, $9);")
                .bind(tenant_id)
                .bind(partner.partner_id.clone())
                .bind(partner.business_name.clone())
                .bind(partner.description.clone())
                .bind(partner.first_name.clone())
                .bind(partner.middle_name.clone())
                .bind(partner.last_name.clone())
                .bind(partner.prefix.clone())
                .bind(partner.suffix.clone())
                .execute(&pool)
                .await
            {
                Err(e) => {
                    error!("Error saving partner record: {:?}", e);
                    return Err("Error saving partner record");
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

    async fn partners_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> Result<Vec<crm_provider::Partner>, &'static str> {
        info!("partners_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, Partnerdata>("select * from crm.partners_fetch($1, $2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching partners: {:?}", e);
                    return Err("Error fetching partners");
                }
                Ok(rows) => {
                    let partners: Vec<crm_provider::Partner> =
                        rows.iter().map(|r| r.0.clone()).collect();

                    return Ok(partners);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn partner_fetch_by_id(
        &self,
        partner_id: &uuid::Uuid,
    ) -> Result<crm_provider::Partner, &'static str> {
        info!("partners_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, Partnerdata>("select * from crm.partner_fetch_by_id($1);")
                .bind(partner_id)
                .fetch_one(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching partner: {:?}", e);
                    return Err("Error fetching partner");
                }
                Ok(r) => {
                    return Ok(r.0.clone());
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn partners_set_active(
        &self,
        partner_ids: &Vec<uuid::Uuid>,
        active: bool,
    ) -> Result<(), &'static str> {
        info!("partners_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call crm.partners_set_active($1, $2);")
                .bind(partner_ids)
                .bind(active)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error setting partner active state: {:?}", e);
                    return Err("Error setting partner active state");
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
    // use rand::{ distr::Alphanumeric, Rng };

    use crm_provider::CrmProvider;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_person_save() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let cp = PostgresCrmProvider::new(&dp);

        let partner_id = uuid::Uuid::new_v4();

        let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
        let tenant_id = tenant.tenant_id();

        let partner = crm_provider::Partner {
            partner_id: partner_id,
            active: true,
            created: chrono::Utc::now(),
            business_name: String::from(format!("test_name_{}", rand::random::<u16>())),
            description: String::from("test_description"),
            first_name: String::from("test_first"),
            middle_name: String::from("test_middle"),
            last_name: String::from("test_last"),
            prefix: String::from("prefix"),
            suffix: String::from("suffix"),
            // gender: 0,
        };

        if let Err(e) = cp.partner_save(&tenant_id, &partner).await {
            error!("Error saving partner record: {:?}", e);
            assert!(false, "Failed to save partner record");
        }

        if let Err(e) = cp.partners_set_active(&vec![partner_id], true).await {
            error!("Error setting partner active state: {:?}", e);
            assert!(false, "Failed to set partner active state");
        }
    }
}
