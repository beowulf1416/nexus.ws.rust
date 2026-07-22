use tracing::{debug, error, info};

use core::future::Future;
use serde::Serialize;
use std::vec::Vec;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationData {
    pub org_id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationNodeData {
    pub org_id: Uuid,
    pub parent_org_id: Option<Uuid>,
    pub active: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: String,
    pub level: i32,
}

pub trait OrganizationsProvider {
    fn save(
        &self,
        tenant_id: &uuid::Uuid,
        org_id: &uuid::Uuid,
        parent_org_id: &uuid::Uuid,
        name: &str,
        description: &str,
        version: &i32,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<OrganizationData>, &'static str>> + Send;

    fn fetch_tree(
        &self,
        tenant_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Vec<OrganizationNodeData>, &'static str>> + Send;

    fn fetch_by_id(
        &self,
        org_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<OrganizationData, &'static str>> + Send;
}
