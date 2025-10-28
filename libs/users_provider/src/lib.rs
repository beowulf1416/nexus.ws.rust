pub struct User {
    pub user_id: uuid::Uuid,
    pub active: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub prefix: String,
    pub suffix: String
}




pub trait UsersProvider {

    fn save(
        &self,
        user_id: &uuid::Uuid,
        first_name: &str,
        middle_name: &str,
        last_name: &str,
        prefix: &str,
        suffix: &str
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn set_active(
        &self,
        user_id: &uuid::Uuid,
        active: &bool
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn fetch(
        &self,
        user_id: &uuid::Uuid
    ) -> impl Future<Output = Result<User, &'static str>> + Send;
}