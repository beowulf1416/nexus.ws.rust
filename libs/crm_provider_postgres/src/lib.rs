#![allow(clippy::needless_return)]

use tracing::{
    info,
    error,
    debug
};

use sqlx::Row;




pub struct PostgresCrmProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresCrmProvider {

    pub fn new(
        dp: &database_provider::DatabaseProvider
    ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}



impl crm_provider::CrmProvider for PostgresCrmProvider {

    async fn person_save(
        &self,
        tenant_id: &uuid::Uuid,
        person: crm_provider::Person
    ) -> Result<(), &'static str> {
        info!("person_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call crm.person_save($1, $2, $3, $4, $5, $6, $7);")
                .bind(tenant_id)
                .bind(person.first_name)
                .bind(person.middle_name)
                .bind(person.last_name)
                .bind(person.prefix)
                .bind(person.suffix)
                .bind(person.gender)
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error saving person record: {:?}", e);
                        return Err("Error saving person record");
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

    use tenants_provider::TenantsProvider;

	#[actix_web::test]
	async fn test_registration() {
	    if let Err(e) = tracing_subscriber::fmt::try_init() {
	        println!("error: {:?}", e);
	    }

	    let cfg = config::Config::from_env();
	    let db_provider = database_provider::DatabaseProvider::new(&cfg);
	    let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));
		let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);

		let people_id = uuid::Uuid::new_v4();

		let tenant_id = tp.get_tenant_id().await.unwrap();
	}
}
