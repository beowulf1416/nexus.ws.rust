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


impl User {

    pub fn nil() -> Self {
        return Self {
            user_id: uuid::Uuid::nil(),
            active: false,
            created: chrono::Utc::now(),
            first_name: String::from(""),
            middle_name: String::from(""),
            last_name: String::from(""),
            prefix: String::from(""),
            suffix: String::from("")
        };
    }

    pub fn is_nil(&self) -> bool {
        return self.user_id.is_nil();
    }
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

    fn add_email(
        &self,
        user_id: &uuid::Uuid,
        email: &str
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn fetch_by_id(
        &self,
        user_id: &uuid::Uuid
    ) -> impl Future<Output = Result<User, &'static str>> + Send;

    fn fetch_by_email(
        &self,
        email: &str
    ) -> impl Future<Output = Result<User, &'static str>> + Send;
}