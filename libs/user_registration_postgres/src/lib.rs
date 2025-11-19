#![allow(clippy::needless_return)]

use tracing::{
    info,
    debug,
    error
};

use sqlx::Row;


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

    async fn fetch_registration_details_by_token(
        &self,
        token: &str
    ) -> Result<user_registration::UserRegistrationDetails, &'static str> {
        info!("fetch_registration_details_by_token");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from user_registration.fetch_registration_details_by_token($1);")
                .bind(token)
                .fetch_one(&pool)
                .await {
                    Ok(row) => {
                        debug!("row: {:?}", row);

                        let register_id: uuid::Uuid = row.get("id");
                        let email: &str = row.get("email");
                        let token: &str = row.get("token");

                        return Ok(user_registration::UserRegistrationDetails::new(
                            &register_id,
                            email,
                            token
                        ));
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

    async fn fetch_registration_details_by_id(
        &self,
        register_id: &uuid::Uuid
    ) -> Result<user_registration::UserRegistrationDetails, &'static str> {
        info!("fetch_registration_details_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from user_registration.fetch_registration_details_by_id($1);")
                .bind(register_id)
                .fetch_one(&pool)
                .await {
                    Ok(row) => {
                        debug!("row: {:?}", row);

                        let register_id: uuid::Uuid = row.get("id");
                        let email: &str = row.get("email");
                        let token: &str = row.get("token");

                        return Ok(user_registration::UserRegistrationDetails::new(
                            &register_id,
                            email,
                            token
                        ));
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


    async fn verify_registration(
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
    use rand::{distr::Alphanumeric, Rng};

    use users_provider::UsersProvider;
    use user_registration::UserRegistrationProvider;
    use auth_provider::AuthProvider;

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
        let mut rng = rand::rng();
        let token: String = (0..50)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect()
            ;

        let ur = PostgresUserRegistrationProvider::new(&dp);

        if let Err(e) = ur.register_user(&register_id, &email, &token).await {
            assert!(false, "error registering user: {}", e);
        }
    }


    #[actix_web::test]
    async fn test_registration() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let register_id = uuid::Uuid::new_v4();
        let email = format!("test_{}@test.com", rand::random::<u16>());
        let mut rng = rand::rng();
        let token: String = (0..50)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect()
            ;

        let ur = PostgresUserRegistrationProvider::new(&dp);

        if let Err(e) = ur.register_user(&register_id, &email, &token).await {
            assert!(false, "error registering user: {}", e);
        }

        if let Err(e) = ur.fetch_registration_details_by_id(&register_id).await {
            assert!(false, "error fetching registration details by id")
        }

        if let Err(e) = ur.fetch_registration_details_by_token(&token).await {
            assert!(false, "error fetching registration details by token")
        }

        if let Err(e) = ur.verify_registration(&register_id, &token).await {
            assert!(false, "error verifying registrations");
        }


        let up = users_provider_postgres::PostgresUsersProvider::new(&dp);
        if let Err(e) = up.save(&register_id, &"", &"", &"", &"", &"").await {
            assert!(false, "error adding user details");
        }

        
        let ap = auth_provider_postgres::PostgresAuthProvider::new(&dp);
        if let Err(e) = ap.add_user_auth_password(&register_id, &email, &"test1test").await {
            assert!(false, "error adding user authentication");
        }

    }
}
