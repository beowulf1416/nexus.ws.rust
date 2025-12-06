use tracing::{
    info,
    error,
    debug
};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Tenant {
    id: uuid::Uuid,
    name: String,
    description: String
}


impl Tenant {

    pub fn new(
        id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> Self {
        return Self {
            id: id.clone(),
            name: String::from(name),
            description: String::from(description)
        };
    }

    pub fn default() -> Self {
        return Self {
            id: uuid::Uuid::nil(),
            name: String::from("default"),
            description: String::from("default")
        }
    }

    pub fn tenant_id(&self) -> uuid::Uuid {
        return self.id.clone();
    }
}
