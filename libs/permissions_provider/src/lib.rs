
use serde::{
    Serialize
};

#[derive(Debug, Clone, Serialize)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: String
}


pub trait PermissionsProvider {


    fn fetch(
        &self,
        filter: &str
    ) -> impl Future<Output = Result<Vec<Permission>, &'static str>> + Send;

    fn fetch_by_id(
        &self,
        id: &i32
    ) -> impl Future<Output = Result<Permission, &'static str>> + Send;

    fn fetch_by_name(
        &self,
        name: &str
    ) -> impl Future<Output = Result<Permission, &'static str>> + Send;
}