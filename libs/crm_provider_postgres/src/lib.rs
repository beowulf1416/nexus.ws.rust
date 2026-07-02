#![allow(clippy::needless_return)]

use tracing::{
    info,
    error,
    // debug
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

    async fn partner_save(
        &self,
        tenant_id: &uuid::Uuid,
        partner: &crm_provider::Partner
    ) -> Result<(), &'static str> {
        info!("partner_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call crm.partner_save($1, $2, $3, $4, $5, $6, $7, $8, $9);")
                .bind(tenant_id)
                .bind(partner.partner_id.clone())
                .bind(partner.business_name.clone())
                .bind(partner.description.clone())
                .bind(partner.first_name.clone())
                .bind(partner.middle_name.clone())
                .bind(partner.last_name.clone())
                .bind(partner.prefix.clone())
                .bind(partner.suffix.clone())
                .execute(&pool)
                .await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error saving partner record: {:?}", e);
                        return Err("Error saving partner record");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    // async fn business_save(
    //     &self,
    //     tenant_id: &uuid::Uuid,
    //     business: &crm_provider::Business
    // ) -> Result<(), &'static str> {
    // 	info!("business_save");

	   //  if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
	   //      match sqlx::query("call crm.business_save($1, $2, $3, $4);")
	   //          .bind(tenant_id)
	   //          .bind(business.business_id.clone())
	   //          .bind(business.name.clone())
	   //          .bind(business.description.clone())
	   //          .execute(&pool)
	   //          .await {
	   //              Ok(_) => {
	   //                  return Ok(());
	   //              }
	   //              Err(e) => {
	   //                  error!("Error saving business record: {:?}", e);
	   //                  return Err("Error saving business record");
	   //              }
	   //          }
	   //  } else {
	   //      error!("No Postgres pool found for 'main'");
	   //      return Err("Unable to get pool for 'main'");
	   //  }
    // }

    async fn partners_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> Result<Vec<crm_provider::Partner>, &'static str> {
   		info!("partners_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from crm.partners_fetch($1, $2);")
                .bind(tenant_id)
                .bind(filter)
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        let partners: Vec<crm_provider::Partner> = rows.iter().map(|r| {
                            let partner_id: uuid::Uuid = r.get("partner_id");
                            let active: bool = r.get("active");
                            let created: chrono::DateTime<chrono::Utc> = r.get("created_at");
                            let business_name: String = r.get("business_name");
                            let description: String = r.get("description");
                            let first_name: String = r.get("first_name");
                            let middle_name: String = r.get("middle_name");
                            let last_name: String = r.get("last_name");
                            let prefix: String = r.get("prefix");
                            let suffix: String = r.get("suffix");

                            return crm_provider::Partner {
                                partner_id,
                                active,
                                created,
                                business_name,
                                description,
                                first_name,
                                middle_name,
                                last_name,
                                prefix,
                                suffix,
                            };

                        }).collect();

                        return Ok(partners);
                    }
                    Err(e) => {
                        error!("Error fetching partners: {:?}", e);
                        return Err("Error fetching partners");
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
    // use rand::{ distr::Alphanumeric, Rng };

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

		let partner_id = uuid::Uuid::new_v4();

		let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
		let tenant_id = tenant.tenant_id();

		let partner = crm_provider::Partner {
			partner_id: partner_id,
			active: true,
			created: chrono::Utc::now(),
			business_name: String::from(format!("test_name_{}", rand::random::<u16>())),
			description: String::from("test_description"),
			first_name: String::from("test_first"),
			middle_name: String::from("test_middle"),
			last_name: String::from("test_last"),
			prefix: String::from("prefix"),
			suffix: String::from("suffix"),
			// gender: 0,
		};

		if let Err(e) = cp.partner_save(&tenant_id, &partner).await {
			error!("Error saving partner record: {:?}", e);
			assert!(false, "Failed to save partner record");
		}
	}

	// #[actix_web::test]
	// async fn test_business_save() {
	//     if let Err(e) = tracing_subscriber::fmt::try_init() {
	//         println!("error: {:?}", e);
	//     }

	//     let cfg = config::Config::from_env();
	//     let db_provider = database_provider::DatabaseProvider::new(&cfg);
	//     let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

	// 	let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
	// 	let cp = PostgresCrmProvider::new(&dp);

	// 	let business_id = uuid::Uuid::new_v4();

	// 	let tenant = tp.tenant_fetch_by_name("tenant_01").await.unwrap();
	// 	let tenant_id = tenant.tenant_id();

	// 	let business = crm_provider::Business {
	// 		business_id: business_id,
	// 		name: String::from(format!("test_name_{}", rand::random::<u16>())),
	// 		description: String::from("test_description")
	// 	};

	// 	if let Err(e) = cp.business_save(&tenant_id, &business).await {
	// 		error!("Error saving business record: {:?}", e);
	// 		assert!(false, "Failed to save business record");
	// 	}
	// }
}
