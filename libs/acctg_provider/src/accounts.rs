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

pub trait AccountsProvider {
    fn account_types_fetch(
        &self,
    ) -> impl Future<Output = Result<Vec<AccountType>, &'static str>> + Send;

    fn account_categories_fetch(
        &self,
    ) -> impl Future<Output = Result<Vec<AccountCategory>, &'static str>> + Send;
}
