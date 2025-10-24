pub mod tenant;


use tracing::{
    info,
    debug,
    error
};

use uuid::Uuid;
use core::future::Future;
use serde::Serialize;
use std::vec::Vec;





pub trait AdminTenantsProvider {

    fn tenants_fetch(
        &self,
        filter: &str
    ) -> impl Future<Output = Result<Vec<tenant::Tenant>, &'static str>> + Send;
}