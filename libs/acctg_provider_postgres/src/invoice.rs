#![allow(clippy::needless_return)]

use sqlx::Row;
use tracing::{debug, error, info};

use acctg_provider::invoice::{Invoice, InvoiceItem, InvoiceProvider, InvoiceType};

pub struct InvoiceProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl InvoiceProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl InvoiceProvider for InvoiceProviderPostgres {
    async fn invoice_types_fetch(&self) -> Result<Vec<InvoiceType>, &'static str> {
        info!("invoice_types_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.invoice_types_fetch();")
                .fetch_all(&pool)
                .await
            {
                Ok(rows) => {
                    let types: Vec<InvoiceType> = rows
                        .iter()
                        .map(|r| {
                            let invoice_type_id: i32 = r.get("invoice_type_id");
                            let name: String = r.get("name");
                            return InvoiceType {
                                id: invoice_type_id,
                                name,
                            };
                        })
                        .collect();

                    return Ok(types);
                }
                Err(e) => {
                    error!("Error fetching invoice types: {:?}", e);
                    return Err("Error fetching invoice types");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn invoice_save(&self, invoice: &Invoice) -> Result<(), &'static str> {
        info!("invoice_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call acctg.invoice_save();")
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error saving invoice: {:?}", e);
                    return Err("Error saving invoice");
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

    use acctg_provider::invoice::InvoiceProvider;

    #[actix_web::test]
    async fn test_countries() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let ipp = InvoiceProviderPostgres::new(&dp);

        if let Err(e) = ipp.invoice_types_fetch().await {
            error!(e);
            assert!(false, "unable to fetch invoice types");
        }
    }
}
