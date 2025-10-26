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
}