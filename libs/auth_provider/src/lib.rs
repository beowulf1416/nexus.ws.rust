pub enum AuthenticationType {
    Password
}


#[derive(Debug, Clone)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub email: String
}



pub trait AuthProvider {

    fn add_user_auth_password(
        &self,
        user_id: &uuid::Uuid,
        email: &str,
        pw: &str
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn user_auth_password_set_active(
        &self,
        user_id: &uuid::Uuid,
        active: bool
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn authenticate_by_password(
        &self,
        email: &str,
        pw: &str
    ) -> impl Future<Output = Result<bool, &'static str>> + Send;

    fn fetch_user_by_id(
        &self,
        user_id: &uuid::Uuid
    ) -> impl Future<Output = Result<User, &'static str>> + Send;
}