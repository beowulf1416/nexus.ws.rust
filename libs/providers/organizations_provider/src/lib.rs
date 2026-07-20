#![allow(clippy::needless_return)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub org_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub parent_org_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationTreeItem {
    pub org_id: uuid::Uuid,
    pub active: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: String,
    pub parent_org_id: Option<uuid::Uuid>,
    pub level: i32,
    pub path: String,
}

pub trait OrganizationsProvider {
    fn organizations_save(
        &self,
        tenant_id: &uuid::Uuid,
        organization: &Organization,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn organizations_fetch_tree(
        &self,
        tenant_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Vec<OrganizationTreeItem>, &'static str>> + Send;
}
