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
        person: &crm_provider::Person
    ) -> Result<(), &'static str> {
        info!("person_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call crm.person_save($1, $2, $3, $4, $5, $6, $7, $8);")
                .bind(tenant_id)
                .bind(person.people_id.clone())
                .bind(person.first_name.clone())
                .bind(person.middle_name.clone())
                .bind(person.last_name.clone())
                .bind(person.prefix.clone())
                .bind(person.suffix.clone())
                .bind(person.gender.clone())
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

    async fn business_save(
        &self,
        tenant_id: &uuid::Uuid,
        business: &crm_provider::Business
    ) -> Result<(), &'static str> {
    info!("business_save");

	    if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
	        match sqlx::query("call crm.business_save($1, $2, $3, $4);")
	            .bind(tenant_id)
	            .bind(business.business_id.clone())
	            .bind(business.name.clone())
	            .bind(business.description.clone())
	            .execute(&pool)
	            .await {
	                Ok(_) => {
	                    return Ok(());
	                }
	                Err(e) => {
	                    error!("Error saving business record: {:?}", e);
	                    return Err("Error saving business record");
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
    use rand::{ distr::Alphanumeric, Rng };

    use tenants_provider::TenantsProvider;
    use crm_provider::CrmProvider;

	#[actix_web::test]
	async fn test_person_save() {
	    if let Err(e) = tracing_subscriber::fmt::try_init() {
	        println!("error: {:?}", e);
	    }

	    let cfg = config::Config::from_env();
	    let db_provider = database_provider::DatabaseProvider::new(&cfg);
	    let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

		let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
		let cp = PostgresCrmProvider::new(&dp);

		let people_id = uuid::Uuid::new_v4();

		let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
		let tenant_id = tenant.tenant_id();

		let person = crm_provider::Person {
			people_id: people_id,
			active: true,
			created: chrono::Utc::now(),
			first_name: String::from("test_first"),
			middle_name: String::from("test_middle"),
			last_name: String::from("test_last"),
			prefix: String::from("prefix"),
			suffix: String::from("suffix"),
			gender: 0,
		};

		if let Err(e) = cp.person_save(&tenant_id, &person).await {
			error!("Error saving person record: {:?}", e);
			assert!(false, "Failed to save person record");
		}
	}

		#[actix_web::test]
		async fn test_business_save() {
		    if let Err(e) = tracing_subscriber::fmt::try_init() {
		        println!("error: {:?}", e);
		    }

		    let cfg = config::Config::from_env();
		    let db_provider = database_provider::DatabaseProvider::new(&cfg);
		    let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

			let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
			let cp = PostgresCrmProvider::new(&dp);

			let business_id = uuid::Uuid::new_v4();

			let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
			let tenant_id = tenant.tenant_id();

			let business = crm_provider::Business {
				business_id: business_id,
				name: String::from("test_name"),
				description: String::from("test_description")
			};

			if let Err(e) = cp.business_save(&tenant_id, &business).await {
				error!("Error saving business record: {:?}", e);
				assert!(false, "Failed to save business record");
			}
		}
}
