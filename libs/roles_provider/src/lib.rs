
pub struct Role {

}


pub trait RolesProvider {

    fn save(
        &self,
        role: &Role
    ) -> impl Future<Output = Result<(), &'static str>> + Send; 

    fn fetch_by_id(
        &self,
        role_id: &uuid::Uuid
    ) -> impl Future<Output = Result<Role, &'static str>> + Send;
}