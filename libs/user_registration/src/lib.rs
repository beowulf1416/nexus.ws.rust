use sqlx::query_as;
use tracing::{
    info,
    debug,
    error
};

use uuid::Uuid;
use core::future::Future;


pub trait UserRegistrationProvider {

    fn register_user(
        &self,
        register_id: &uuid::Uuid,
        email: &str,
        token: &str
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn fetch_registration_details_by_token(
        &self,
        token: &str
    ) -> impl Future<Output = Result<UserRegistrationDetails, &'static str>> + Send;

    fn fetch_registration_details_by_id(
        &self,
        register_id: &uuid::Uuid
    ) -> impl Future<Output = Result<UserRegistrationDetails, &'static str>> + Send;
}


#[derive(Debug, Clone)]
pub struct UserRegistrationDetails {
    register_id: uuid::Uuid,
    email: String,
    token: String
}


impl UserRegistrationDetails {

    pub fn new(
        register_id: &uuid::Uuid,
        email: &str,
        token: &str
    ) -> Self {
        return Self {
            register_id: register_id.clone(),
            email: String::from(email),
            token: String::from(token)
        };
    }

    pub fn register_id(&self) -> uuid::Uuid {
        return self.register_id.clone();
    }

    pub fn email(&self) -> String {
        return self.email.clone();
    }

    pub fn token(&self) -> String {
        return self.token.clone();
    }
}


pub struct UserRegistration {
    dp: database_provider::DatabaseProvider
}


impl UserRegistration {

    // pub fn new(
    //     dp: &database_provider::DatabaseProvider
    // ) -> Self {
    //     return Self {
    //         dp: dp.clone()
    //     };
    // }

    // pub async fn register_user(
    //     &self,
    //     register_id: &uuid::Uuid,
    //     email: &str,
    //     token: &str
    // ) -> Result<(), &'static str> {
    //     info!("register_user");

    //     if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
    //         match sqlx::query("call user_registration.register_user($1, $2, $3);")
    //             .bind(register_id)
    //             .bind(email)
    //             .bind(token)
    //             .execute(&pool)
    //             .await {
    //                 Ok(_) => {
    //                     return Ok(());
    //                 }
    //                 Err(e) => {
    //                     error!("Error registering user: {:?}", e);
    //                     return Err("Error registering user");
    //                 }
    //             }
    //     } else {
    //         error!("No Postgres pool found for 'main'");
    //         return Err("Unable to get pool for 'main'");
    //     }
    // }


    // pub async fn get_details(
    //     &self,
    //     token: &str
    // ) -> Result<(), &'static str> {
    //     info!("get_details");
    //     debug!("token: {:?}", token);

    //     if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
    //         match sqlx::query("select * from user_registration.get_details($1);")
    //             .bind(token)
    //             .fetch_one(&pool)
    //             .await {
    //                 Ok(row) => {
    //                     debug!("row: {:?}", row);
    //                     return Ok(());
    //                 }
    //                 Err(e) => {
    //                     error!("Error verifying user registration: {:?}", e);
    //                     return Err("Error verifying user registration");
    //                 }
    //             }
    //     } else {
    //         error!("No Postgres pool found for 'main'");
    //         return Err("Unable to get pool for 'main'");
    //     }
    // }


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