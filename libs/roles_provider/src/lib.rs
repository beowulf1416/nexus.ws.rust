use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Role {
    pub role_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub created: chrono::DateTime<chrono::Utc>
}


pub trait RolesProvider {

    fn save(
        &self,
        tenant_id: &uuid::Uuid,
        role: &Role
    ) -> impl Future<Output = Result<(), &'static str>> + Send; 

    fn fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str
    ) -> impl Future<Output = Result<Vec<Role>, &'static str>> + Send;
}