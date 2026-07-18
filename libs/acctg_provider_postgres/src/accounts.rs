#![allow(clippy::needless_return)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, prelude::FromRow};
use std::collections::HashMap;
use tracing::{debug, error, info};

use acctg_provider::accounts::{
    Account, AccountCategory, AccountNode, AccountType, AccountsProvider,
};

pub struct AccountTypeItem(pub AccountType);

impl<'r> FromRow<'r, PgRow> for AccountTypeItem {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(AccountType {
            account_type_id: row.get("account_type_id"),
            name: row.get("name"),
        }));
    }
}

#[derive(Debug)]
pub struct AccountCategoryItem(pub AccountCategory);

impl<'r> FromRow<'r, PgRow> for AccountCategoryItem {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self(AccountCategory {
            account_category_id: row.get("acct_category_id"),
            name: row.get("name"),
            sub_name: row.get("sub_name"),
        }));
    }
}

#[derive(Debug)]
pub struct AccountItem {
    pub account_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub name: String,
    pub level: i32,
    pub path: String,
}

impl<'r> FromRow<'r, PgRow> for AccountItem {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        return Ok(Self {
            account_id: row.get("account_id"),
            parent_id: row.get("parent_account_id"),
            name: row.get("name"),
            level: row.get("level"),
            path: row.get("path"),
        });
    }
}

pub struct AccountsProviderPostgres {
    dp: database_provider::DatabaseProvider,
}

impl AccountsProviderPostgres {
    pub fn new(dp: &database_provider::DatabaseProvider) -> Self {
        return Self { dp: dp.clone() };
    }

    fn build_tree(&self, accounts: &Vec<AccountItem>) -> Vec<AccountNode> {
        info!("build_tree");

        // place accounts into a hash map for quick lookup
        let mut account_map: HashMap<Option<uuid::Uuid>, AccountNode> = HashMap::new();
        for a in accounts {
            account_map
                .entry(a.parent_id)
                .or_insert(AccountNode {
                    account_id: uuid::Uuid::nil(),
                    active: true,
                    account_type_id: 0,
                    account_category_id: 0,
                    name: String::from("root"),
                    code: String::from("root"),
                    description: String::from("root"),
                    children: Vec::<AccountNode>::new(),
                })
                .children
                .push(AccountNode {
                    account_id: a.account_id,
                    active: true,
                    account_type_id: 0,
                    account_category_id: 0,
                    name: a.name.clone(),
                    code: a.name.clone(),
                    description: a.name.clone(),
                    children: Vec::<AccountNode>::new(),
                });
        }

        // debug!("account_tree: {:?}", account_map);

        return account_map
            .get(&Some(uuid::Uuid::nil()))
            .cloned()
            .unwrap()
            .children;
    }
}

impl AccountsProvider for AccountsProviderPostgres {
    async fn account_types_fetch(&self) -> Result<Vec<AccountType>, &'static str> {
        info!("account_types_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, AccountTypeItem>("select * from acctg.account_types_fetch();")
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching account types: {:?}", e);
                    return Err("Error fetching account types");
                }
                Ok(rows) => {
                    return Ok(rows.into_iter().map(|r| r.0).collect());
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
            match sqlx::query_as::<_, AccountCategoryItem>(
                "select * from acctg.account_categories_fetch();",
            )
            .fetch_all(&pool)
            .await
            {
                Err(e) => {
                    error!("Error fetching account categories: {:?}", e);
                    return Err("Error fetching account categories");
                }
                Ok(rows) => {
                    return Ok(rows.iter().map(|r| r.0.clone()).collect());
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn accounts_fetch_all(
        &self,
        tenant_id: &uuid::Uuid,
    ) -> Result<Vec<Account>, &'static str> {
        info!("accounts_fetch_all");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.accounts_fetch_all($1);")
                .bind(tenant_id)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching accounts: {:?}", e);
                    return Err("Error fetching accounts");
                }
                Ok(rows) => {
                    let accounts: Vec<Account> = rows
                        .iter()
                        .map(|r| {
                            let account_id: uuid::Uuid = r.get("account_id");
                            let active: bool = r.get("active");
                            let account_type_id: i16 = r.get("account_type_id");
                            let account_category_id: i16 = r.get("account_category_id");
                            let name: String = r.get("name");
                            let code: String = r.get("code");
                            let description: String = r.get("description");
                            return Account {
                                account_id,
                                active,
                                account_type_id,
                                account_category_id,
                                name,
                                code,
                                description,
                            };
                        })
                        .collect();

                    return Ok(accounts);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn accounts_fetch_by_type(
        &self,
        tenant_id: &uuid::Uuid,
        type_id: &i16,
    ) -> Result<Vec<Account>, &'static str> {
        info!("accounts_fetch_by_type");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.accounts_fetch_by_type($1, $2);")
                .bind(tenant_id)
                .bind(type_id)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching accounts: {:?}", e);
                    return Err("Error fetching accounts");
                }
                Ok(rows) => {
                    let accounts: Vec<Account> = rows
                        .iter()
                        .map(|r| {
                            let account_id: uuid::Uuid = r.get("account_id");
                            let active: bool = r.get("active");
                            let account_type_id: i16 = r.get("account_type_id");
                            let account_category_id: i16 = r.get("account_category_id");
                            let name: String = r.get("name");
                            let code: String = r.get("code");
                            let description: String = r.get("description");
                            return Account {
                                account_id,
                                active,
                                account_type_id,
                                account_category_id,
                                name,
                                code,
                                description,
                            };
                        })
                        .collect();

                    return Ok(accounts);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn accounts_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        account_type_id: &i16,
        filter: &str,
    ) -> Result<Vec<Account>, &'static str> {
        info!("accounts_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.accounts_fetch($1, $2, $3);")
                .bind(tenant_id)
                .bind(account_type_id)
                .bind(filter)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching accounts: {:?}", e);
                    return Err("Error fetching accounts");
                }
                Ok(rows) => {
                    let accounts: Vec<Account> = rows
                        .iter()
                        .map(|r| {
                            let account_id: uuid::Uuid = r.get("account_id");
                            let active: bool = r.get("active");
                            let account_type_id: i16 = r.get("account_type_id");
                            let account_category_id: i16 = r.get("account_category_id");
                            let name: String = r.get("name");
                            let code: String = r.get("code");
                            let description: String = r.get("description");
                            return Account {
                                account_id,
                                active,
                                account_type_id,
                                account_category_id,
                                name,
                                code,
                                description,
                            };
                        })
                        .collect();
                    // debug!("accounts: {:?}", accounts);
                    return Ok(accounts);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn accounts_fetch_tree(
        &self,
        tenant_id: &uuid::Uuid,
    ) -> Result<Vec<AccountNode>, &'static str> {
        info!("accounts_fetch_tree");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query_as::<_, AccountItem>("select * from acctg.accounts_fetch_tree($1);")
                .bind(tenant_id)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching accounts: {:?}", e);
                    return Err("Error fetching accounts");
                }
                Ok(accounts) => {
                    // debug!("accounts: {:?}", accounts);

                    let account_nodes = self.build_tree(&accounts);

                    return Ok(account_nodes);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn account_fetch(&self, account_id: &uuid::Uuid) -> Result<Account, &'static str> {
        info!("account_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.account_fetch($1);")
                .bind(account_id)
                .fetch_one(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching account: {:?}", e);
                    return Err("Error fetching account");
                }
                Ok(r) => {
                    if r.is_empty() {
                        return Err("Account not found");
                    }
                    let account_id: uuid::Uuid = r.get("account_id");
                    let active: bool = r.get("active");
                    let account_type_id: i16 = r.get("account_type_id");
                    let account_category_id: i16 = r.get("account_category_id");
                    let name: String = r.get("name");
                    let code: String = r.get("code");
                    let description: String = r.get("description");

                    let account = Account {
                        account_id,
                        active,
                        account_type_id,
                        account_category_id,
                        name,
                        code,
                        description,
                    };
                    // debug!("accounts: {:?}", accounts);
                    return Ok(account);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn account_fetch_children(
        &self,
        account_id: &uuid::Uuid,
    ) -> Result<Vec<Account>, &'static str> {
        info!("accounts_fetch");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("select * from acctg.account_fetch_children($1);")
                .bind(account_id)
                .fetch_all(&pool)
                .await
            {
                Err(e) => {
                    error!("Error fetching accounts: {:?}", e);
                    return Err("Error fetching accounts");
                }
                Ok(rows) => {
                    let accounts: Vec<Account> = rows
                        .iter()
                        .map(|r| {
                            let account_id: uuid::Uuid = r.get("account_id");
                            let active: bool = r.get("active");
                            let account_type_id: i16 = r.get("account_type_id");
                            let account_category_id: i16 = r.get("account_category_id");
                            let name: String = r.get("name");
                            let code: String = r.get("code");
                            let description: String = r.get("description");
                            return Account {
                                account_id,
                                active,
                                account_type_id,
                                account_category_id,
                                name,
                                code,
                                description,
                            };
                        })
                        .collect();
                    // debug!("accounts: {:?}", accounts);
                    return Ok(accounts);
                }
            }
        } else {
            error!("No Postgres pool found for 'main'");
            return Err("Unable to get pool for 'main'");
        }
    }

    async fn account_save(
        &self,
        tenant_id: &uuid::Uuid,
        account: &Account,
    ) -> Result<(), &'static str> {
        info!("account_save");

        if let Some(database_provider::DatabaseType::Postgres(pool)) = self.dp.get_pool("main") {
            match sqlx::query("call acctg.account_save($1, $2, $3, $4, $5, $6, $7);")
                .bind(&tenant_id)
                .bind(&account.account_id)
                .bind(account.account_type_id)
                .bind(account.account_category_id)
                // .bind(account.active)
                .bind(&account.code)
                .bind(&account.name)
                .bind(&account.description)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    error!("Error saving accounts: {:?}", e);
                    return Err("Error saving accounts");
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

        let name = format!("test_{}", rand::random::<u16>());

        let tenant_id = tp
            .tenant_fetch_by_name("tenant_01")
            .await
            .unwrap()
            .tenant_id();

        if let Err(e) = app.account_types_fetch().await {
            error!(e);
            assert!(false, "unable to fetch account types");
        }

        if let Err(e) = app.account_categories_fetch().await {
            error!(e);
            assert!(false, "unable to fetch account categories");
        }

        let account_id = uuid::Uuid::new_v4();

        if let Err(e) = app
            .account_save(
                &tenant_id,
                &Account {
                    account_id: account_id,
                    active: true,
                    account_type_id: 1,
                    account_category_id: 1,
                    name: name.clone(),
                    code: name.clone(),
                    description: name.clone(),
                },
            )
            .await
        {
            error!(e);
            assert!(false, "unable to save account");
        }

        if let Err(e) = app.account_fetch(&account_id).await {
            error!(e);
            assert!(false, "unable to fetch account");
        }

        if let Err(e) = app.account_fetch_children(&account_id).await {
            error!(e);
            assert!(false, "unable to fetch account");
        }

        if let Err(e) = app.accounts_fetch_all(&tenant_id).await {
            error!(e);
            assert!(false, "unable to fetch accounts");
        }

        if let Err(e) = app.accounts_fetch_by_type(&tenant_id, &1).await {
            error!(e);
            assert!(false, "unable to fetch accounts by type");
        }

        if let Err(e) = app.accounts_fetch(&tenant_id, &1, &"%").await {
            error!(e);
            assert!(false, "unable to fetch accounts by filter");
        }

        if let Err(e) = app.accounts_fetch_tree(&tenant_id).await {
            error!(e);
            assert!(false, "unable to fetch accounts tree");
        }
    }
}
