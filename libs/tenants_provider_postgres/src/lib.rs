#![allow(clippy::needless_return)]

use tracing::{
    info,
    debug,
    error
};

use sqlx::{Row, types::chrono};
use std::vec::Vec;


pub struct PostgresTenantsProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresTenantsProvider {
    pub fn new(
        dp: &database_provider::DatabaseProvider
     ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}



impl tenants_provider::TenantsProvider for PostgresTenantsProvider {

    async fn tenants_fetch_by_id(
        &self,
        tenant_id: &uuid::Uuid
    ) -> Result<tenants_provider::Tenant, &'static str> {
        info!("tenants_fetch_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from tenants.tenants_fetch_by_id($1);")
                .bind(tenant_id)
                .fetch_one(&pool)
                .await {
                    Ok(r) => {
                        debug!("//todo: {:?}", r);

                        let tenant_id: uuid::Uuid = r.get("tenant_id");
                        let active: bool = r.get("active");
                        let created: chrono::DateTime<chrono::Utc> = r.get("created");
                        let name: String = r.get("name");
                        let description: String = r.get("description");



                        return Ok(tenants_provider::Tenant::new(
                            &tenant_id,
                            active,
                            &created,
                            &name,
                            &description
                        ));
                    }
                    Err(e) => {
                        error!("Error fetching tenant record: {:?}", e);
                        return Err("Error fetching tenant record");
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
        description: &str
    ) -> Result<(), &'static str> {
        info!("tenant_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenant_save($1,$2,$3);")
                .bind(tenant_id)
                .bind(name)
                .bind(description)
                .execute(&pool)
                .await {
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
        active: &bool
    ) -> Result<(), &'static str> {
        info!("tenant_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenant_set_active($1,$2);")
                .bind(tenant_id)
                .bind(active)
                .execute(&pool)
                .await {
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
        active: &bool
    ) -> Result<(), &'static str> {
        info!("tenants_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenants_set_active($1,$2);")
                .bind(tenant_ids)
                .bind(active)
                .execute(&pool)
                .await {
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
        filter: &str
    ) -> Result<Vec<tenants_provider::Tenant>, &'static str> {
        info!("tenants_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from tenants.tenants_fetch($1);")
                .bind(filter)
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        let tenants: Vec<tenants_provider::Tenant> = rows.iter().map(|r| {
                            let tenant_id: uuid::Uuid = r.get("tenant_id");
                            let active: bool = r.get("active");
                            let created: chrono::DateTime<chrono::Utc> = r.get("created");
                            let name: String = r.get("name");
                            let description: String = r.get("description");

                            return tenants_provider::Tenant::new(
                                &tenant_id,
                                active,
                                &created,
                                name.as_str(),
                                description.as_str()
                            );
                        }).collect();
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

        if let Err(e) = tp.tenant_save(&tenant_id, &name, &description).await {
            error!(e);
            assert!(false, "unable to save tenant record");
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