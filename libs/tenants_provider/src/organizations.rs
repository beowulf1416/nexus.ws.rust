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

    fn fetch_by_id(
        &self,
        org_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<OrganizationData, &'static str>> + Send;
}
