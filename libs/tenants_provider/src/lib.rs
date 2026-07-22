pub mod organizations;

use tracing::{debug, error, info};

use core::future::Future;
use serde::Serialize;
use std::vec::Vec;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Tenant {
    pub id: uuid::Uuid,
    pub active: bool,
    pub version: i32,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: String,
}

impl Tenant {
    pub fn new(
        tenant_id: &uuid::Uuid,
        active: bool,
        version: i32,
        created: &chrono::DateTime<chrono::Utc>,
        updated: &chrono::DateTime<chrono::Utc>,
        name: &str,
        description: &str,
    ) -> Self {
        return Self {
            id: tenant_id.clone(),
            active,
            version,
            updated: updated.clone(),
            created: created.clone(),
            name: String::from(name),
            description: String::from(description),
        };
    }

    pub fn default() -> Self {
        return Self {
            id: uuid::Uuid::nil(),
            active: true,
            version: 0,
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            name: String::from("default"),
            description: String::from("default"),
        };
    }

    pub fn tenant_id(&self) -> uuid::Uuid {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn description(&self) -> String {
        return self.description.clone();
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Permission {
    pub id: i32,
    pub name: String,
}

impl Permission {
    pub fn new(permission_id: &i32, name: &str) -> Self {
        return Self {
            id: permission_id.clone(),
            name: String::from(name),
        };
    }

    pub fn id(&self) -> i32 {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}

pub trait TenantsProvider {
    fn tenants_fetch_by_id(
        &self,
        tenant_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Tenant, &'static str>> + Send;

    fn tenant_fetch_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Tenant, &'static str>> + Send;

    fn tenant_save(
        &self,
        tenant_id: &uuid::Uuid,
        name: &str,
        description: &str,
        version: &i32,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn tenant_set_active(
        &self,
        tenant_id: &uuid::Uuid,
        active: &bool,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn tenants_set_active(
        &self,
        tenant_ids: &Vec<uuid::Uuid>,
        active: &bool,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn tenants_fetch(
        &self,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<Tenant>, &'static str>> + Send;

    fn tenant_user_tenants_fetch(
        &self,
        user_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Vec<Tenant>, &'static str>> + Send;

    fn tenant_user_permissions_fetch(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Vec<Permission>, &'static str>> + Send;
}
