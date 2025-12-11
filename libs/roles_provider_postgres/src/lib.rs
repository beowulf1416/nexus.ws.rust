#![allow(clippy::needless_return)]

use tracing::{
    info,
    error,
    debug
};

use sqlx::Row;



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


    async fn set_active(
        &self,
        role_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), &'static str> {
        info!("save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.role_set_active($1,$2);")
                .bind(role_id)
                .bind(active)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error setting active state of role: {:?}", e);
                        return Err("Error setting active state of role");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str
    ) -> Result<Vec<roles_provider::Role>, &'static str> {
        info!("fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from tenants.roles_fetch($1,$2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await {
                    Ok(rows ) => {
                        let roles: Vec<roles_provider::Role> = rows.iter().map(|r| {
                            let role_id: uuid::Uuid = r.get("role_id");
                            let active: bool = r.get("active");
                            let created: chrono::DateTime<chrono::Utc> = r.get("created");
                            let name: String = r.get("name");
                            let description: String = r.get("description");

                            return roles_provider::Role {
                                role_id,
                                name,
                                description,
                                active,
                                created
                            };
                        }).collect();
                        return Ok(roles);
                    }
                    Err(e) => {
                        error!("Error fetching role records: {:?}", e);
                        return Err("Error fetching role records");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn assign_users(
        &self,
        role_ids: &Vec<uuid::Uuid>,
        user_ids: &Vec<uuid::Uuid>
    ) -> Result<(), &'static str> {
        info!("assign_users");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.role_users_add($1, $2);")
                .bind(role_ids)
                .bind(user_ids)
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error assigning users to role: {:?}", e);
                        return Err("Error assigning users to role");
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn revoke_users(
        &self,
        role_ids: &Vec<uuid::Uuid>,
        user_ids: &Vec<uuid::Uuid>
    ) -> Result<(), &'static str> {
        info!("revoke_users");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.role_users_remove($1, $2);")
                .bind(role_ids)
                .bind(user_ids)
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error revoking users from role: {:?}", e);
                        return Err("Error revoking users from role");
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn role_user_set_active(
        &self,
        role_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), &'static str> {
        info!("revoke_users");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.role_user_set_active($1, $2, $3);")
                .bind(role_id)
                .bind(user_id)
                .bind(active)
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error setting active state of role: {:?}", e);
                        return Err("Error setting active state of role");
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn tenant_user_set_active(
        &self,
        tenant_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), &'static str> {
        info!("revoke_users");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.tenant_user_set_active($1, $2, $3);")
                .bind(tenant_id)
                .bind(user_id)
                .bind(active)
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error setting active state of tenant users: {:?}", e);
                        return Err("Error setting active state of tenant users");
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    
    async fn assign_permissions(
        &self,
        role_ids: &Vec<uuid::Uuid>,
        permission_ids: &Vec<i32>
    ) -> Result<(), &'static str> {
        info!("assign_permissions");    

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.role_permissions_add($1, $2);")
                .bind(role_ids)
                .bind(permission_ids)
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error assigning permissions for role: {:?}", e);
                        return Err("Error assigning permissions for role");
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn revoke_permissions(
        &self,
        role_ids: &Vec<uuid::Uuid>,
        permission_ids: &Vec<i32>
    ) -> Result<(), &'static str> {
        info!("revoke_permissions");    

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.role_permissions_remove($1, $2);")
                .bind(role_ids)
                .bind(permission_ids)
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error revoking permissions from role: {:?}", e);
                        return Err("Error revoking permissions from role");
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn role_permission_set_active(
        &self,
        role_id: &uuid::Uuid,
        permission_ids: &Vec<i32>,
        active: bool
    ) -> Result<(), &'static str> {
        info!("revoke_permissions");    

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call tenants.role_permission_set_active($1, $2, $3);")
                .bind(role_id)
                .bind(permission_ids)
                .bind(active)
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error setting active state of role permission: {:?}", e);
                        return Err("Error setting active state of role permission");
                    }
                    Ok(_) => {
                        return Ok(());
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

        if let Err(e) = rp.set_active(&role_id, &true).await {
            error!("unable to set active state of role: {:?}", e);
            assert!(false, "unable to set active state of role");
        }

        if let Err(e) = rp.fetch(&tenant_id, "%").await {
            error!("unable to fetch roles: {}", e);
            assert!(false, "unable to fetch roles");
        }

        if let Err(e) = rp.assign_permissions(
            &vec!(role_id),
            &vec!(1)
        ).await {
            error!("unable to assign permission to role: {}", e);
            assert!(false, "unable to assign permission to role");
        }

        if let Err(e) = rp.revoke_permissions(
            &vec!(role_id),
            &vec!(1)
        ).await {
            error!("unable to revoke permission from role: {}", e);
            assert!(false, "unable to revoke permission from role");
        }

        if let Err(e) = rp.role_permission_set_active(
            &role_id,
            &vec!(1,2),
            true
        ).await {
            error!("unable to revoke permission from role: {}", e);
            assert!(false, "unable to revoke permission from role");
        }
    }
}
