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

        // convert to hashmap for efficient lookup
        let mut nodes: HashMap<Option<uuid::Uuid>, Vec<AccountNode>> = HashMap::new();
        // insert null uuid key value
        nodes.insert(Some(uuid::Uuid::nil()), Vec::new());

        for a in accounts {
            // debug!("account: {:?}", a);
            nodes
                .entry(a.parent_id)
                .or_insert(Vec::new()) // if entry is not found, use an empty vector
                // insert the account node into the vector for the parent
                .push(AccountNode {
                    account_id: a.account_id,
                    active: true,
                    account_type_id: 0,
                    account_category_id: 0,
                    name: a.name.clone(),
                    code: a.name.clone(),
                    description: a.name.clone(),
                    children: Vec::new(),
                    level: a.level,
                });
        }
        // debug!("nodes: {:?}", nodes);

        fn build(
            parent_id: Option<uuid::Uuid>,
            children: &mut HashMap<Option<uuid::Uuid>, Vec<AccountNode>>,
        ) -> Vec<AccountNode> {
            // info!("build: parent_id={:?}", parent_id);
            let result = children
                .remove(&parent_id)
                .unwrap_or_default()
                .into_iter()
                .map(|r| AccountNode {
                    account_id: r.account_id,
                    active: r.active,
                    account_type_id: r.account_type_id,
                    account_category_id: r.account_category_id,
                    name: r.name.clone(),
                    code: r.code.clone(),
                    description: r.description.clone(),
                    level: r.level,
                    // call this recursively to build the children
                    children: build(Some(r.account_id), children),
                })
                .collect::<Vec<_>>();

            return result;
        }

        let result = build(Some(uuid::Uuid::nil()), &mut nodes);
        // debug!("result: {:?}", result);
        return result;
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
                    // debug!("before accounts_fetch_tree: {:?}", accounts);
                    let account_nodes = self.build_tree(&accounts);
                    // debug!("after accounts_fetch_tree: {:?}", account_nodes);

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

        // test tree building
        let asset_acct_id = uuid::Uuid::new_v4();
        let asset_child_id_1 = uuid::Uuid::new_v4();
        let asset_child_id_2 = uuid::Uuid::new_v4();
        let asset_child_id_1_1 = uuid::Uuid::new_v4();

        let liability_acct_id = uuid::Uuid::new_v4();
        let liability_child_id_1 = uuid::Uuid::new_v4();

        let mut accounts = vec![
            AccountItem {
                account_id: asset_acct_id,
                parent_id: Some(uuid::Uuid::nil()),
                name: "asset".to_string(),
                level: 0,
                path: "asset".to_string(),
            },
            AccountItem {
                account_id: asset_child_id_1,
                parent_id: Some(asset_acct_id),
                name: "asset child 1".to_string(),
                level: 1,
                path: "asset/asset child 1".to_string(),
            },
            AccountItem {
                account_id: asset_child_id_1_1,
                parent_id: Some(asset_child_id_1),
                name: "asset child 1 1".to_string(),
                level: 2,
                path: "asset/asset child 1/asset child 1 1".to_string(),
            },
            AccountItem {
                account_id: asset_child_id_2,
                parent_id: Some(asset_acct_id),
                name: "asset child 2".to_string(),
                level: 1,
                path: "asset/asset child 2".to_string(),
            },
            AccountItem {
                account_id: liability_acct_id,
                parent_id: Some(uuid::Uuid::nil()),
                name: "liability".to_string(),
                level: 0,
                path: "liability".to_string(),
            },
            AccountItem {
                account_id: liability_child_id_1,
                parent_id: Some(liability_acct_id),
                name: "liability child 1".to_string(),
                level: 1,
                path: "liability/liability child 1".to_string(),
            },
        ];
        // debug!("before tree_test: {:?}", accounts);
        let result = app.build_tree(&accounts);
        // debug!("after tree_test: {:?}", result);
        assert_eq!(result.len(), 2);
    }
}
