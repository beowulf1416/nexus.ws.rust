use tracing::{
    info,
    debug,
    error
};

use uuid::Uuid;
use core::future::Future;
use serde::Serialize;
use std::vec::Vec;


pub struct Tenant {
    id: uuid::Uuid,
    name: String
}


impl Tenant {

    pub fn new(
        tenant_id: &uuid::Uuid,
        name: &str
    ) -> Self {
        return Self {
            id: tenant_id.clone(),
            name: String::from(name)
        };
    }


    pub fn tenant_id(&self) -> uuid::Uuid {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}







pub trait AdminTenantsProvider {

    fn tenants_fetch(
        &self,
        filter: &str
    ) -> impl Future<Output = Result<Vec<Tenant>, &'static str>> + Send;

    fn tenant_save(
        &self,
        tenant_id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
}