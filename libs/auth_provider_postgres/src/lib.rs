use tracing::{
    info,
    debug,
    error
};

use sqlx::Row;

const AUTH_TYPE_PW: i32 = 1;


pub struct PostgresAuthProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresAuthProvider {
    pub fn new(
        dp: &database_provider::DatabaseProvider
     ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}



impl auth_provider::AuthProvider for PostgresAuthProvider {

    async fn add_user_auth_password(
        &self,
        user_id: &uuid::Uuid,
        email: &str,
        pw: &str
    ) -> Result<(), &'static str> {
        info!("add_user_auth_password");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call auth.user_auth_password_save($1,$2,$3);")
                .bind(user_id)
                .bind(email)
                .bind(pw)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error adding user authentication using password: {:?}", e);
                        return Err("Error adding user authentication using password");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn user_auth_password_set_active(
        &self,
        user_id: &uuid::Uuid,
        active: bool
    ) -> Result<(), &'static str> {
        info!("user_auth_password_set_active");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call auth.user_auth_password_set_active($1,$2);")
                .bind(user_id)
                .bind(active)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error setting user authentication using password active: {:?}", e);
                        return Err("Error setting user authentication using password active");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn authenticate_by_password(
        &self,
        email: &str,
        pw: &str
    ) -> Result<bool, &'static str> {
        info!("authenticate");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from auth.user_auth_password_authenticate($1,$2);")
                .bind(email)
                .bind(pw)
                .fetch_one(&pool)
                .await {
                    Ok(row) => {
                        debug!("{:?}", row);

                        let authentic = row.get("user_auth_password_authenticate");
                        return Ok(authentic);
                    }
                    Err(e) => {
                        error!("Error user authentication using password: {:?}", e);
                        return Err("Error user authentication using password");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn fetch_user_by_id(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<auth_provider::User, &'static str> {
        info!("fetch_user_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from auth.user_auth_password_fetch($1);")
                .bind(user_id)
                .fetch_one(&pool)
                .await {
                    Ok(row) => {
                        debug!("{:?}", row);

                        let user_id: uuid::Uuid = row.get("user_id");
                        let email: String = row.get("email");

                        return Ok(auth_provider::User {
                            user_id,
                            email
                        });
                    }
                    Err(e) => {
                        error!("Error user authentication using password: {:?}", e);
                        return Err("Error user authentication using password");
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
    use auth_provider::AuthProvider;

    use users_provider::UsersProvider;
    use users_provider_postgres::PostgresUsersProvider;

    #[actix_web::test]
    async fn test_user_auth_password() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let user_id = uuid::Uuid::new_v4();
        let email = format!("test_{}@test.com", rand::random::<u16>());
        let pw = "test1test";

        let up = PostgresUsersProvider::new(&dp);

        if let Err(e) = up.save(&user_id, &"", &"", &"", &"", &"").await {
            error!(e);
            assert!(false, "unable to add user authentication using password");
        }

        let ap = PostgresAuthProvider::new(&dp);

        if let Err(e) = ap.add_user_auth_password(&user_id, &email, &pw).await {
            error!(e);
            assert!(false, "unable to add user authentication using password");
        }

        if let Err(e) = ap.user_auth_password_set_active(&user_id, true).await {
            error!(e);
            assert!(false, "unable to set user authentication using password active");
        }

        if let Err(e) = ap.authenticate_by_password(&email, &pw).await {
            error!(e);
            assert!(false, "unable to authenticate using password");
        }

        if let Err(e) = ap.fetch_user_by_id(&user_id).await {
            error!(e);
            assert!(false, "unable to fetch user by id");
        }
    }
}
