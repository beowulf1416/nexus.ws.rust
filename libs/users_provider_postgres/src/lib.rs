#![allow(clippy::needless_return)]

use tracing::{
    info,
    debug,
    error
};

use sqlx::Row;

use chrono::{
    DateTime,
    NaiveDateTime
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


    async fn set_active_multiple(
        &self,
        user_ids: &Vec<uuid::Uuid>,
        active: &bool
    ) -> Result<(), &'static str> {
        info!("set_active_multiple");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call users.users_set_active_multiple($1,$2);")
                .bind(user_ids)
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



    async fn add_email(
        &self,
        user_id: &uuid::Uuid,
        email: &str
    ) -> Result<(), &'static str> {
        info!("add_email");    

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call users.user_emails_add($1,$2);")
                .bind(user_id)
                .bind(email)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error adding user email: {:?}", e);
                        return Err("Error adding user email");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn fetch_by_id(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<users_provider::User, &'static str> {
        info!("fetch_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from users.users_fetch_by_id($1);")
                .bind(user_id)
                .fetch_one(&pool)
                .await {
                    Ok(row) => {
                        debug!("//todo {:?}", row);

                        let user_id: uuid::Uuid = row.get("user_id");
                        let active: bool = row.get("active");
                        let created: chrono::DateTime<chrono::Utc> = row.get("created");
                        let first_name: String = row.get("first_name");
                        let middle_name: String = row.get("middle_name");
                        let last_name: String = row.get("last_name");
                        let prefix: String = row.get("prefix");
                        let suffix: String = row.get("suffix");
                        let email: String = row.get("email");

                        return Ok(users_provider::User { 
                            user_id,
                            active,
                            created,
                            first_name,
                            middle_name,
                            last_name,
                            prefix,
                            suffix,
                            email
                        });
                    }
                    Err(e) => {
                        error!("Error fetching user details using id: {:?}", e);
                        return Err("Error fetching user details using id");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }



    async fn fetch_by_email(
        &self,
        email: &str
    ) -> Result<users_provider::User, &'static str> {
        info!("fetch_by_email");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from users.users_fetch_by_email($1);")
                .bind(email)
                .fetch_one(&pool)
                .await {
                    Ok(row) => {
                        debug!("//todo {:?}", row);

                        let user_id: uuid::Uuid = row.get("user_id");
                        let active: bool = row.get("active");
                        let created: chrono::DateTime<chrono::Utc> = row.get("created");
                        let first_name: String = row.get("first_name");
                        let middle_name: String = row.get("middle_name");
                        let last_name: String = row.get("last_name");
                        let prefix: String = row.get("prefix");
                        let suffix: String = row.get("suffix");
                        let email: String = row.get("email");

                        return Ok(users_provider::User { 
                            user_id,
                            active,
                            created,
                            first_name,
                            middle_name,
                            last_name,
                            prefix,
                            suffix,
                            email
                        });
                    }
                    Err(e) => {
                        error!("Error fetching user details using email: {:?}", e);
                        return Err("Error fetching user details using email");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn fetch(
        &self,
        filter: &str
    ) -> Result<Vec<users_provider::User>, &'static str> {
        info!("fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from users.users_fetch($1);")
                .bind(filter)
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        debug!("//todo {:?}", rows);

                        let users: Vec<users_provider::User> = rows.iter().map(|r| {
                            let user_id: uuid::Uuid = r.get("user_id");
                            let active: bool = r.get("active");
                            let created: chrono::DateTime<chrono::Utc> = r.get("created");
                            let first_name: String = r.get("first_name");
                            let middle_name: String = r.get("middle_name");
                            let last_name: String = r.get("last_name");
                            let prefix: String = r.get("prefix");
                            let suffix: String = r.get("suffix");
                            let email: String = r.get("email");

                            return users_provider::User::new( 
                                &user_id,
                                &active,
                                &created,
                                &first_name,
                                &middle_name,
                                &last_name,
                                &prefix,
                                &suffix,
                                &email
                            );
                        }).collect();

                        return Ok(users);
                    }
                    Err(e) => {
                        error!("Error fetching users: {:?}", e);
                        return Err("Error fetching users");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn tenant_users_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str
    ) -> Result<std::vec::Vec<users_provider::User>, &'static str> {
        info!("tenant_users_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from tenants.tenant_users_fetch($1, $2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        let tenants: Vec<users_provider::User> = rows.iter().map(|r| {
                            let user_id: uuid::Uuid = r.get("user_id");
                            let active: bool = r.get("active");
                            let created: chrono::DateTime<chrono::Utc> = r.get("created");
                            let first_name: String = r.get("first_name");
                            let middle_name: String = r.get("middle_name");
                            let last_name: String = r.get("last_name");
                            let prefix: String = r.get("prefix");
                            let suffix: String = r.get("suffix");
                            let email: String = r.get("email");

                            return users_provider::User::new(
                                &user_id,
                                &active,
                                &created,
                                &first_name,
                                &middle_name,
                                &last_name,
                                &prefix,
                                &suffix,
                                &email
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



    async fn tenant_user_save(
        &self,
        tenant_id: &uuid::Uuid,
        user_id: &uuid::Uuid
    ) -> Result<(), &'static str> {
        info!("tenant_user_save");
        
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

    async fn tenant_assign(
        &self,
        user_ids: &Vec<uuid::Uuid>,
        tenant_ids: &Vec<uuid::Uuid>
    ) -> Result<(), &'static str> {
        info!("tenant_assign");
        
        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenant_users_assign($1,$2);")
                .bind(tenant_ids)
                .bind(user_ids)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error assigning user to tenant: {:?}", e);
                        return Err("Error assigning user to tenant");
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
    use tenants_provider::TenantsProvider;

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
        let email = format!("test_{}@test.com", rand::random::<u16>());

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

        if let Err(e) = up.set_active_multiple(
            &vec!(user_id),
            &true
        ).await {
            error!(e);
            assert!(false, "unable to set multiple user active state");
        }

        if let Err(e) = up.add_email(&user_id, &email).await {
            error!(e);
            assert!(false, "unable to add user email");
        }

        if let Err(e) = up.fetch_by_id(&user_id).await {
            error!(e);
            assert!(false, "unable to fetch user by id");
        }

        if let Err(e) = up.fetch_by_email(&email).await {
            error!(e);
            assert!(false, "unable to fetch user by email");
        }

        if let Err(e) = up.fetch("%").await {
            error!(e);
            assert!(false, "unable to fetch users");
        }


        let tenant_id = uuid::Uuid::new_v4();
        let name = format!("test_{}", rand::random::<u16>());
        let description = "users_provider_postgres_test";

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        if let Err(e) = tp.tenant_save(&tenant_id, &name, &description).await {
            error!(e);
            assert!(false, "unable to save tenant into in users_provider_postgres_test");
        }

        if let Err(e) = up.tenant_user_save(&tenant_id, &user_id).await {
            error!(e);
            assert!(false, "unable to save tenant user");
        }

        if let Err(e) = up.tenant_users_fetch(&tenant_id, &"%").await {
            error!(e);
            assert!(false, "unable to fetch tenant users");
        }


        if let Err(e) = up.tenant_assign(
            &vec![user_id], 
            &vec![tenant_id]
        ).await {
            error!(e);
            assert!(false, "unable to fetch assign users to tenants");
        }
    }
}
