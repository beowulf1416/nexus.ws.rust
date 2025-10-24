use tracing::{
    info,
    debug,
    error
};

use sqlx::Row;
use std::vec::Vec;


pub struct PostgresAdminTenantsProvider {
    dp: database_provider::DatabaseProvider
}


impl PostgresAdminTenantsProvider {
    pub fn new(
        dp: &database_provider::DatabaseProvider
     ) -> Self {
        return Self {
            dp: dp.clone()
        };
    }
}



impl admin_tenants::AdminTenantsProvider for PostgresAdminTenantsProvider {

        async fn tenants_fetch(
            &self,
            filter: &str
        ) -> Result<Vec<admin_tenants::tenant::Tenant>, &'static str> {
        
        return Ok(Vec::new());
    }
}