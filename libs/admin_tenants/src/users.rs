use serde::Serialize;

pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub prefix: String,
    pub suffix: String
}


impl User  {

    pub fn new(
        id: &uuid::Uuid,
        first_name: &str,
        middle_name: &str,
        last_name: &str,
        prefix: &str,
        suffix: &str
    ) -> Self {
        return Self {
            id: id.clone(),
            first_name: first_name.to_string(),
            middle_name: middle_name.to_string(),
            last_name: last_name.to_string(),
            prefix: prefix.to_string(),
            suffix: suffix.to_string()
        };
    }
}



pub trait UsersProvider {

    fn users_fetch(
        &self,
        tenant_id: &uuid::Uuid,
        filter: &str
    ) -> impl Future<Output = Result<std::vec::Vec<User>, &'static str>> + Send;


    fn user_save(
        &self,
        tenant_id: &uuid::Uuid,
        user_id: &uuid::Uuid
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
}