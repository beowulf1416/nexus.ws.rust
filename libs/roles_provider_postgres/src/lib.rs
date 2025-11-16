use tracing::{
    info,
    error,
    debug
};



pub struct PostgresRolesProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresRolesProvider {
    pub fn new(
        dp: &database_provider::DatabaseProvider
     ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}


impl roles_provider::RolesProvider for PostgresRolesProvider {

    async fn save(
        &self,
        tenant_id: &uuid::Uuid,
        role: &roles_provider::Role
    ) -> Result<(), &'static str> {
        info!("save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.role_save($1,$2,$3,$4);")
                .bind(tenant_id)
                .bind(role.role_id)
                .bind(role.name.clone())
                .bind(role.description.clone())
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error saving role record: {:?}", e);
                        return Err("Error saving role record");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    // async fn fetch_by_id(
    //     &self,
    //     role_id: &uuid::Uuid
    // ) -> Result<roles_provider::Role, &'static str> {
        
    // }
}



#[cfg(test)]
mod tests {
    use super::*;

    use roles_provider::RolesProvider;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_roles() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tenant_id = uuid::Uuid::new_v4();
        let tenant_name = format!("test_{}", rand::random::<u16>());
        let tenant_description = "roles_provider_postgres_test";

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

        if let Err(e) = tp.tenant_save(&tenant_id, &tenant_name, &tenant_description).await {
            error!("unable to create tenant: {:?}", e);
            assert!(false, "unable to create tenant");
        }

        let role_id = uuid::Uuid::new_v4();
        let role_name = format!("test_{}", rand::random::<u16>());
        let role_description = "roles_provider_postgres_test";

        let role = roles_provider::Role {
            role_id,
            name: role_name,
            description: String::from(role_description),
            active: true,
            created: chrono::Utc::now()
        };

        let rp = PostgresRolesProvider::new(&dp);

        if let Err(e) = rp.save(&tenant_id, &role).await {
            error!("unable to create role: {:?}", e);
            assert!(false, "unable to create role");
        }
    }
}
