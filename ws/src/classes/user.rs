use tracing::{
    info,
    error,
    debug
};



#[derive(Debug, Clone)]
pub struct User {
    user_id: uuid::Uuid,
    tenant_id: uuid::Uuid,
    name: String
}


impl User {

    pub fn new(
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid,
        name: &str) -> Self {
        return Self {
            user_id: user_id.clone(),
            tenant_id: tenant_id.clone(),
            name: String::from(name)
        };
    }

    pub fn anonymous() -> Self {
        return Self {
            user_id: uuid::Uuid::nil(),
            tenant_id: uuid::Uuid::nil(),
            name: String::from("")
        };
    }

    pub fn user_id(&self) -> uuid::Uuid {
        return self.user_id;
    }

    pub fn tenant_id(&self) -> uuid::Uuid {
        return self.tenant_id;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn is_anonymous(&self) -> bool {
        return self.user_id.is_nil();
    }

    pub fn is_authenticated(&self) -> bool {
        return !self.user_id.is_nil();
    }
}