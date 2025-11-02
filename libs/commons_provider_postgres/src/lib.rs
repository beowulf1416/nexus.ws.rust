use tracing::{
    info,
    error,
    debug
};

use sqlx::Row;



pub struct PostgresCommonsProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresCommonsProvider {

    pub fn new(
        dp: &database_provider::DatabaseProvider
    ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}



impl commons_provider::CommonsProvider for PostgresCommonsProvider {

    async fn fetch_countries(&self) -> Result<Vec<commons_provider::Country>, &'static str> {
        info!("fetch_countries");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from common.countries_fetch_all();")
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        let tenants: Vec<commons_provider::Country> = rows.iter().map(|r| {
                            let country_id: i32 = r.get("iso_3166_1_numeric");
                            let name: String = r.get("official_name_en");
                            let code_2: String = r.get("iso_3166_1_alpha_2");
                            let code_3: String = r.get("iso_3166_1_alpha_3");

                            return commons_provider::Country { 
                                id: country_id, 
                                name,
                                code_2,
                                code_3
                            };

                        }).collect();

                        return Ok(tenants);
                    }
                    Err(e) => {
                        error!("Error fetching countries: {:?}", e);
                        return Err("Error fetching countries");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn fetch_currencies(&self) -> Result<Vec<commons_provider::Currency>, &'static str> {
        info!("fetch_currencies");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from common.currencies_fetch_all();")
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        let currencies: Vec<commons_provider::Currency> = rows.iter().map(|r| {
                            let currency_id: i32 = r.get("id");
                            let currency: String = r.get("currency");
                            let unit_text: String = r.get("unit_text");
                            let symbol: Option<String> = r.get("symbol");

                            return commons_provider::Currency { 
                                id: currency_id.clone(), 
                                name: String::from(currency),
                                unit_text: String::from(unit_text),
                                symbol: symbol
                            };

                        }).collect();

                        return Ok(currencies);
                    }
                    Err(e) => {
                        error!("Error fetching currencies: {:?}", e);
                        return Err("Error fetching currencies");
                    }
                }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }


    async fn fetch_genders(&self) -> Result<Vec<commons_provider::Gender>, &'static str> {
        info!("fetch_genders");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from common.genders_fetch_all();")
                .fetch_all(&pool)
                .await {
                    Ok(rows) => {
                        let genders: Vec<commons_provider::Gender> = rows.iter().map(|r| {
                            let gender_id: i16 = r.get("id");
                            let name: String = r.get("name");

                            return commons_provider::Gender { 
                                id: gender_id.clone(), 
                                name: String::from(name)
                            };

                        }).collect();

                        return Ok(genders);
                    }
                    Err(e) => {
                        error!("Error fetching genders: {:?}", e);
                        return Err("Error fetching genders");
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

    use commons_provider::CommonsProvider;


    #[actix_web::test]
    async fn test_countries() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let cp = PostgresCommonsProvider::new(&dp);

        if let Err(e) = cp.fetch_countries().await {
            error!(e);
            assert!(false, "unable to fetch countries");
        }

        if let Err(e) = cp.fetch_currencies().await {
            error!(e);
            assert!(false, "unable to fetch currencies");
        }

        if let Err(e) = cp.fetch_genders().await {
            error!(e);
            assert!(false, "unable to fetch genders");
        }
    }
}
