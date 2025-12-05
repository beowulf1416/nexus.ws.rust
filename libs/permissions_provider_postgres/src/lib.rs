use tracing::{
    info,
    error,
    debug,
};

use sqlx::Row;


pub struct PostgresPermissionsProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresPermissionsProvider {

    pub fn new(
        dp: &database_provider::DatabaseProvider
    ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}


impl permissions_provider::PermissionsProvider for PostgresPermissionsProvider {

    async fn fetch(
        &self,
        filter: &str
    ) -> Result<Vec<permissions_provider::Permission>, &'static str> {
        info!("fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from permissions.permissions_fetch($1);")
                .bind(filter)
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        let permissions: Vec<permissions_provider::Permission> = rows.iter().map(|r| {
                            let id: i32 = r.get("id");
                            let name: String = r.get("name");
                            let description: String = r.get("description");

                            return permissions_provider::Permission { 
                                id, 
                                name,
                                description
                            };

                        }).collect();

                        return Ok(permissions);
                    }
                    Err(e) => {
                        error!("Error fetching permissions: {:?}", e);
                        return Err("Error fetching permissions");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn fetch_by_id(
        &self,
        id: &i32
    ) -> Result<permissions_provider::Permission, &'static str> {
        info!("fetch_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from permissions.permission_fetch_by_id($1);")
                .bind(id)
                .fetch_one(&pool)
                .await {
                    Ok(r) => {
                        let id: i32 = r.get("id");
                        let name: String = r.get("name");
                        let description: String = r.get("description");

                        return Ok(permissions_provider::Permission { 
                            id, 
                            name,
                            description
                        });
                    }
                    Err(e) => {
                        error!("Error fetching permissions: {:?}", e);
                        return Err("Error fetching permissions");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn fetch_by_name(
        &self,
        name: &str
    ) -> Result<permissions_provider::Permission, &'static str> {
        info!("fetch_by_id");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from permissions.permission_fetch_by_name($1);")
                .bind(name)
                .fetch_one(&pool)
                .await {
                    Ok(r) => {
                        let id: i32 = r.get("id");
                        let name: String = r.get("name");
                        let description: String = r.get("description");

                        return Ok(permissions_provider::Permission { 
                            id, 
                            name,
                            description
                        });
                    }
                    Err(e) => {
                        error!("Error fetching permissions: {:?}", e);
                        return Err("Error fetching permissions");
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
    use permissions_provider::PermissionsProvider;

    use super::*;

    #[actix_web::test]
    async fn it_works() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let pp = PostgresPermissionsProvider::new(&dp);

        if let Err(e) = pp.fetch("%").await {
            error!(e);
            assert!(false, "unable to fetch permissions");
        }

        if let Err(e) = pp.fetch_by_id(&1).await {
            error!(e);
            assert!(false, "unable to fetch permission by id");
        }

        if let Err(e) = pp.fetch_by_name(&"tenant.save").await {
            error!(e);
            assert!(false, "unable to fetch permission by name");
        }
    }
}
