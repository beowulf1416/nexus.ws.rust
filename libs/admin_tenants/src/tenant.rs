use crate::tenant;

pub struct Tenant {
    id: uuid::Uuid,
    name: String
}


impl Tenant {

    pub fn new(
        tenant_id: &uuid::Uuid,
        name: &str
    ) -> Self {
        return Self {
            id: tenant_id.clone(),
            name: String::from(name)
        };
    }


    pub fn tenant_id(&self) -> uuid::Uuid {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}