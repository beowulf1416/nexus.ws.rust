use tracing::{
    info,
    debug,
    error
};

use uuid::Uuid;
use core::future::Future;
use serde::Serialize;
use std::vec::Vec;

#[derive(Debug, Serialize)]
pub struct Tenant {
    id: uuid::Uuid,
    active: bool,
    created: chrono::DateTime<chrono::Utc>,
    name: String,
    description: String
}


impl Tenant {

    pub fn new(
        tenant_id: &uuid::Uuid,
        active: bool,
        created: &chrono::DateTime<chrono::Utc>,
        name: &str,
        description: &str
    ) -> Self {
        return Self {
            id: tenant_id.clone(),
            active,
            created: created.clone(),
            name: String::from(name),
            description: String::from(description)
        };
    }

    pub fn default() -> Self {
        return Self {
            id: uuid::Uuid::nil(),
            active: true,
            created: chrono::Utc::now(),
            name: String::from("default"),
            description: String::from("default")
        };
    }


    pub fn tenant_id(&self) -> uuid::Uuid {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}



pub trait TenantsProvider {

    // fn save(
    //     &self,
    //     tenant: &Tenant
    // ) -> impl Future<Output = Result<(), &'static str>> + Send;

    // fn fetch_by_id(
    //     &self,
    //     tenant_id: &uuid::Uuid
    // ) -> impl Future<Output = Result<Tenant, &'static str>> + Send;


    fn tenants_fetch_by_id(
        &self,
        tenant_id: &uuid::Uuid
    ) -> impl Future<Output = Result<Tenant, &'static str>> + Send;

    fn tenant_save(
        &self,
        tenant_id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn tenant_set_active(
        &self,
        tenant_id: &uuid::Uuid,
        active: bool
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn tenants_fetch(
        &self,
        filter: &str
    ) -> impl Future<Output = Result<Vec<Tenant>, &'static str>> + Send;
}