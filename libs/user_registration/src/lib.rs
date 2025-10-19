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
        id: &uuid::Uuid,
        email: &str
    ) -> Result<(), &'static str> {
        info!("register_user");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            // let uuid_id = sqlx::types::Uuid::from_bytes(id.as_bytes().clone());
            let result = sqlx::query("call user_registration.register_user($1, $2);")
                .bind(id)
                .bind(email)
                .fetch_one(&pool)
                .await;
            
            debug!("Result: {:?}", result);
        }
        return Ok(());
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
