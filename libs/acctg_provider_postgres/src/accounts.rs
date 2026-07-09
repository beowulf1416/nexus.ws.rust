#![allow(clippy::needless_return)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use tracing::{error, info};

use acctg_provider::accounts::{AccountCategory, AccountType, AccountsProvider};

pub struct AccountsProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl AccountsProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }
}

impl AccountsProvider for AccountsProviderPostgres {
    async fn account_types_fetch(&self) -> Result<Vec<AccountType>, &'static str> {
        info!("account_types_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.account_types_fetch();")
                .fetch_all(&pool)
                .await
            {
                Ok(rows) => {
                    let types: Vec<AccountType> = rows
                        .iter()
                        .map(|r| {
                            let account_type_id: i16 = r.get("account_type_id");
                            let name: String = r.get("name");
                            return AccountType {
                                account_type_id: account_type_id,
                                name,
                            };
                        })
                        .collect();

                    return Ok(types);
                }
                Err(e) => {
                    error!("Error fetching account types: {:?}", e);
                    return Err("Error fetching account types");
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn account_categories_fetch(
        &self,
    ) -> Result<Vec<acctg_provider::accounts::AccountCategory>, &'static str> {
        info!("account_categories_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.account_categories_fetch();")
                .fetch_all(&pool)
                .await
            {
                Ok(rows) => {
                    let categories: Vec<AccountCategory> = rows
                        .iter()
                        .map(|r| {
                            let account_category_id: i16 = r.get("acct_category_id");
                            let name: String = r.get("name");
                            let sub_name: String = r.get("sub_name");
                            return AccountCategory {
                                account_category_id,
                                name,
                                sub_name,
                            };
                        })
                        .collect();

                    return Ok(categories);
                }
                Err(e) => {
                    error!("Error fetching categories types: {:?}", e);
                    return Err("Error fetching categories types");
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

    use acctg_provider::accounts::AccountsProvider;
    use tenants_provider::TenantsProvider;

    #[actix_web::test]
    async fn test_accounts() {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            println!("error: {:?}", e);
        }

        let cfg = config::Config::from_env();
        let db_provider = database_provider::DatabaseProvider::new(&cfg);
        let dp = actix_web::web::Data::new(std::sync::Arc::new(db_provider));

        let tp = tenants_provider_postgres::PostgresTenantsProvider::new(&dp);
        let app = AccountsProviderPostgres::new(&dp);

        if let Err(e) = app.account_types_fetch().await {
            error!(e);
            assert!(false, "unable to fetch account types");
        }

        if let Err(e) = app.account_categories_fetch().await {
            error!(e);
            assert!(false, "unable to fetch account categories");
        }
    }
}
