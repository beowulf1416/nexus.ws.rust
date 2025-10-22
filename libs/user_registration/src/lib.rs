use sqlx::query_as;
use tracing::{
    info,
    debug,
    error
};

use uuid::Uuid;

pub struct UserRegistration {
    dp: database_provider::DatabaseProvider
}


impl UserRegistration {

    pub fn new(
        dp: &database_provider::DatabaseProvider
    ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }

    pub async fn register_user(
        &self,
        register_id: &uuid::Uuid,
        email: &str,
        token: &str
    ) -> Result<(), &'static str> {
        info!("register_user");

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
    }


    pub async fn get_details(
        &self,
        token: &str
    ) -> Result<(), &'static str> {
        info!("get_details");
        debug!("token: {:?}", token);

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from user_registration.get_details($1);")
                .bind(token)
                .fetch_one(&pool)
                .await {
                    Ok(row) => {
                        debug!("row: {:?}", row);
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error verifying user registration: {:?}", e);
                        return Err("Error verifying user registration");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    pub async fn verify_registration(
        &self,
        register_id: &uuid::Uuid,
        token: &str
    ) -> Result<(), &'static str> {
        info!("verify_registration");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call user_registration.verify_registration($1, $2);")
                .bind(register_id)
                .bind(token)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error verifying user registration: {:?}", e);
                        return Err("Error verifying user registration");
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
    use rand::Rng;

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

        let ur = UserRegistration::new(&dp);

        match ur.register_user(&register_id, &email, &token).await {
            Ok(_) => {
                assert!(true);
            },
            Err(e) => {
                assert!(false, "error registering user: {}", e);
            }   
        }

    }
}
