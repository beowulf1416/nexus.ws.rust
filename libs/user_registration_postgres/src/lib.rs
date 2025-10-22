use tracing::{
    info,
    error
};


pub struct PostgresUserRegistrationProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresUserRegistrationProvider {
    pub fn new(
        dp: &database_provider::DatabaseProvider
     ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}

impl user_registration::UserRegistrationProvider for PostgresUserRegistrationProvider {

    async fn register_user(
        &self,
        register_id: &uuid::Uuid,
        email: &str,
        token: &str
    ) -> Result<(), &'static str> {
        info!("register_user");

        // |register_id, email, token| async move {
            if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
                match sqlx::query("call user_registration.register_user($1, $2, $3);")
                    .bind(register_id)
                    .bind(email)
                    .bind(token)
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
        // }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    use user_registration::UserRegistrationProvider;

    const TOKEN_LENGTH: usize = 32;

    #[actix_web::test]
    async fn test_register_user() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let register_id = uuid::Uuid::new_v4();
        let email = format!("test_{}@test.com", rand::random::<u16>());
        let token: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(TOKEN_LENGTH)
            .map(char::from)
            .collect()
            ;

        let ur = PostgresUserRegistrationProvider::new(&dp);

        if let Err(e) = ur.register_user(&register_id, &email, &token).await {
            assert!(false, "error registering user: {}", e);
        }
    }
}
