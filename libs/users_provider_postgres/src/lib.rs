use tracing::{
    info,
    debug,
    error
};

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



impl users_provider::UsersProvider for PostgresUsersProvider {

    async fn save(
            &self,
            user_id: &uuid::Uuid,
            first_name: &str,
            middle_name: &str,
            last_name: &str,
            prefix: &str,
            suffix: &str
        ) -> Result<(), &'static str> {
            info!("save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call users.user_save($1,$2,$3,$4,$5,$6);")
                .bind(user_id)
                .bind(first_name)
                .bind(middle_name)
                .bind(last_name)
                .bind(prefix)
                .bind(suffix)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error registering user: {:?}", e);
                        return Err("Error registering user");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn set_active(
        &self,
        user_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), &'static str> {
        info!("set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call users.users_set_active($1,$2);")
                .bind(user_id)
                .bind(active)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error setting user active status: {:?}", e);
                        return Err("Error setting user active status");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn fetch(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<users_provider::User, &'static str> {
            info!("save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from users.users_fetch_by_id($1);")
                .bind(user_id)
                .fetch_one(&pool)
                .await {
                    Ok(row) => {
                        debug!("//todo {:?}", row);
                        return Ok(users_provider::User { 
                            user_id: uuid::Uuid::new_v4(),
                            active: false,
                            created: chrono::Utc::now(),
                            first_name: String::from("test"),
                            middle_name: String::from("test"),
                            last_name: String::from("test"),
                            prefix: String::from("test"),
                            suffix: String::from("test") 
                        });
                    }
                    Err(e) => {
                        error!("Error registering user: {:?}", e);
                        return Err("Error registering user");
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
    use users_provider::UsersProvider;

    use super::*;

    #[actix_web::test]
    async fn test_user() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let user_id = uuid::Uuid::new_v4();
        let first_name = "test_first";
        let middle_name = "test_middle";
        let last_name = "test_last";
        let prefix = "test_prefix";
        let suffix = "test_suffix";

        let up = PostgresUsersProvider::new(&dp);
        if let Err(e) = up.save(&user_id, &first_name, &middle_name, &last_name, &prefix, &suffix).await {
            error!(e);
            assert!(false, "unable to save user");
        }

        if let Err(e) = up.set_active(&user_id, &true).await {
            error!(e);
            assert!(false, "unable to set user active state");
        }

        if let Err(e) = up.fetch(&user_id).await {
            error!(e);
            assert!(false, "unable to fetch user");
        }
    }
}
