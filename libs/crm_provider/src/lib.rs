#![allow(clippy::needless_return)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Partner {
    pub partner_id: uuid::Uuid,

    pub active: bool,
    pub created: chrono::DateTime<chrono::Utc>,

    pub business_name: String,
    pub description: String,

    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub prefix: String,
    pub suffix: String,
    // pub gender: i16,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Business {
// 	pub business_id: uuid::Uuid,
// 	pub name: String,
// 	pub description: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Partner {
// 	pub partner_id: uuid::Uuid,
// 	pub name: String,
// 	pub description: String,
// }

pub trait CrmProvider {
    fn partner_save(
        &self,
        tenant_id: &uuid::Uuid,
        partner: &Partner,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    // fn business_save(
    // 	&self,
    // 	tenant_id: &uuid::Uuid,
    // 	business: &Business
    // ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn partners_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<Partner>, &'static str>> + Send;

    fn partners_set_active(
        &self,
        partner_ids: &Vec<uuid::Uuid>,
        active: bool,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
}
