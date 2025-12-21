use tracing::{
    info,
    error
};

pub struct PostgresFileProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresFileProvider {
    pub fn new(
        dp: &database_provider::DatabaseProvider
     ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}


impl file_provider::FileProvider for PostgresFileProvider {

    async fn folder_add(
        &self,
        tenant_id: &uuid::Uuid,
        folder: &file_provider::Folder
    ) -> Result<(), &'static str> {
        info!("folder_add");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call files.folder_add($1,$2,$3);")
                .bind(tenant_id)
                .bind(folder.folder_id)
                .bind(folder.name.clone())
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error adding folder record: {:?}", e);
                        return Err("Error adding folder record");
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
        }

        return Err("No database pool found");
    }

    async fn folder_get(
        &self,
        folder_id: &uuid::Uuid
    ) -> Result<file_provider::Folder, &'static str> {
        info!("folder_get");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from fp.folder_get($1)")
                .bind(folder_id)
                .fetch_one(&pool)
                .await {
                    Err(e) => {
                        error!("Error getting folder record: {:?}", e);
                        return Err("Error getting folder record");
                    }
                    Ok(record) => {
                        return Ok(file_provider::Folder {
                            folder_id: record.0,
                            name: record.1
                        });
                    }
                }
        }

        return Err("No database pool found");
    }

    async fn folder_list_files(
        &self,
        folder_id: &uuid::Uuid
    ) -> Result<Vec<file_provider::File>, &'static str> {
        info!("folder_list_files");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from files.folder_list_files($1)")
                .bind(folder_id)
                .fetch_all(&pool)
                .await {
                    Err(e) => {
                        error!("Error listing files in folder: {:?}", e);
                        return Err("Error listing files in folder");
                    }
                    Ok(records) => {
                        let files: Vec<file_provider::File> = records.into_iter().map(|record| {
                            file_provider::File {
                                file_id: record.0,
                                name: record.1
                            }
                        }).collect();
                        return Ok(files);
                    }
                }
        }

        return Err("No database pool found");
    }

    async fn file_add(
        &self,
        tenant_id: &uuid::Uuid,
        file: &file_provider::File
    ) -> Result<(), &'static str> {
        info!("file_add");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call files.file_add($1,$2,$3);")
                .bind(tenant_id)
                .bind(file.file_id)
                .bind(file.name.clone())
                .execute(&pool)
                .await {
                    Err(e) => {
                        error!("Error adding folder record: {:?}", e);
                        return Err("Error adding folder record");
                    }
                    Ok(_) => {
                        return Ok(());
                    }
                }
        }

        return Err("No database pool found");
    }

    async fn file_get(
        &self,
        file_id: &uuid::Uuid
    ) -> Result<file_provider::File, &'static str> {
        info!("file_get");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from files.file_get($1)")
                .bind(file_id)
                .fetch_one(&pool)
                .await {
                    Err(e) => {
                        error!("Error getting folder record: {:?}", e);
                        return Err("Error getting folder record");
                    }
                    Ok(record) => {
                        return Ok(file_provider::File {
                            file_id: record.0,
                            name: record.1
                        });
                    }
                }
        }

        return Err("No database pool found");
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    use file_provider::FileProvider;
    use tenants_provider::TenantsProvider;


    #[actix_web::test]
    async fn test_postgres_file_provider() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let tenant = tp.tenant_fetch_by_name("default").await.unwrap();
        let tenant_id = tenant.tenant_id();


        let fpp = PostgresFileProvider::new(&dp);

        let folder = file_provider::Folder {
            folder_id: uuid::Uuid::new_v4(),
            name: "Test Folder".to_string()
        };

        if let Err(e) = fpp.folder_add(&tenant_id, &folder).await {
            error!("error adding folder: {:?}", e);
            assert!(false, "error adding folder");
        };
    }
}
