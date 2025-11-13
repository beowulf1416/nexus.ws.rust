use serde::Serialize;

#[derive(Debug, Serialize)]
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

    pub fn new(
        user_id: &uuid::Uuid,
        active: &bool,
        created: &chrono::DateTime<chrono::Utc>,
        first_name: &str,
        middle_name: &str,
        last_name: &str,
        prefix: &str,
        suffix: &str
    ) -> Self {
        return Self {
            user_id: user_id.clone(),
            active: active.clone(),
            created: created.clone(),
            first_name: first_name.to_string(),
            middle_name: middle_name.to_string(),
            last_name: last_name.to_string(),
            prefix: prefix.to_string(),
            suffix: suffix.to_string()
        };
    }

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

    fn fetch(
        &self,
        filter: &str
    ) -> impl Future<Output = Result<Vec<User>, &'static str>> + Send;


    fn tenant_users_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str
    ) -> impl Future<Output = Result<std::vec::Vec<User>, &'static str>> + Send;


    fn tenant_user_save(
        &self,
        tenant_id: &uuid::Uuid,
        user_id: &uuid::Uuid
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
}