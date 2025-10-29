pub enum AuthenticationType {
    Password
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
}