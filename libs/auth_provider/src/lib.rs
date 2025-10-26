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

    fn authenticate(
        &self,
        user_id: &uuid::Uuid,
        pw: &str
    ) -> impl Future<Output = Result<bool, &'static str>> + Send;
}