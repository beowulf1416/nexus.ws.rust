use tracing::{
    info,
    debug,
    error
};

use sqlx::{Row, types::chrono};
use std::vec::Vec;



pub struct PostgresUsersProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresUsersProvider {
    pub fn new(
        dp: &database_provider::DatabaseProvider
     ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}


impl admin_tenants::users::UsersProvider for PostgresUsersProvider {

    async fn users_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str
    ) -> Result<std::vec::Vec<admin_tenants::users::User>, &'static str> {
        info!("users_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from tenants.tenant_users_fetch($1, $2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        let tenants: Vec<admin_tenants::users::User> = rows.iter().map(|r| {
                            let user_id: uuid::Uuid = r.get("user_id");
                            let first_name: String = r.get("first_name");
                            let middle_name: String = r.get("middle_name");
                            let last_name: String = r.get("last_name");
                            let prefix: String = r.get("prefix");
                            let suffix: String = r.get("suffix");

                            return admin_tenants::users::User::new(
                                &user_id,
                                &first_name,
                                &middle_name,
                                &last_name,
                                &prefix,
                                &suffix
                            );
                        }).collect();
                        return Ok(tenants);
                    }
                    Err(e) => {
                        error!("Error fetching tenant users records: {:?}", e);
                        return Err("Error fetching tenant users records");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }



    async fn user_save(
        &self,
        tenant_id: &uuid::Uuid,
        user_id: &uuid::Uuid
    ) -> Result<(), &'static str> {
        info!("user_save");
        
        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenant_user_save($1,$2);")
                .bind(tenant_id)
                .bind(user_id)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error adding tenant user: {:?}", e);
                        return Err("Error adding tenant user");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }
}