pub struct Tenant {

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