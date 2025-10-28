pub struct Tenant {
    pub id: uuid::Uuid,
    pub name: String
}


impl Tenant {

    pub fn nil() -> Self {
        return Self {
            id: uuid::Uuid::nil(),
            name: String::from("")
        };
    }

    pub fn is_nil(&self) -> bool {
        return self.id.is_nil();
    }
}



pub trait TenantsProvider {

    fn save(
        &self,
        tenant: &Tenant
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn fetch_by_id(
        &self,
        tenant_id: &uuid::Uuid
    ) -> impl Future<Output = Result<Tenant, &'static str>> + Send;
}