#![allow(clippy::needless_return)]

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountType {
    pub account_type_id: i16,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCategory {
    pub account_category_id: i16,
    pub name: String,
    pub sub_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: uuid::Uuid,
    pub active: bool,
    pub account_type_id: i16,
    pub account_category_id: i16,
    pub name: String,
    pub code: String,
    pub description: String,
}

pub trait AccountsProvider {
    fn account_types_fetch(
        &self,
    ) -> impl Future<Output = Result<Vec<AccountType>, &'static str>> + Send;

    fn account_categories_fetch(
        &self,
    ) -> impl Future<Output = Result<Vec<AccountCategory>, &'static str>> + Send;

    fn accounts_fetch_all(
        &self,
        tenant_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Vec<Account>, &'static str>> + Send;

    fn accounts_fetch_by_type(
        &self,
        tenant_id: &uuid::Uuid,
        type_id: &i16,
    ) -> impl Future<Output = Result<Vec<Account>, &'static str>> + Send;

    fn accounts_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<Account>, &'static str>> + Send;

    fn account_save(
        &self,
        tenant_id: &uuid::Uuid,
        account: &Account,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
}
