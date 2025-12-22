pub struct Folder {
    pub folder_id: uuid::Uuid,
    pub name: String
}

pub struct File {
    pub file_id: uuid::Uuid,
    pub name: String
}


pub trait FileProvider {

    fn folder_add(
        &self,
        tenant_id: &uuid::Uuid,
        folder: &Folder
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn folder_get(
        &self,
        folder_id: &uuid::Uuid
    ) -> impl Future<Output = Result<Folder, &'static str>> + Send;

    fn folder_list_files(
        &self,
        folder_id: &uuid::Uuid
    ) -> impl Future<Output = Result<Vec<File>, &'static str>> + Send;

    fn file_add(
        &self,
        tenant_id: &uuid::Uuid,
        folder_id: &uuid::Uuid,
        file: &File
    ) -> impl Future<Output = Result<(), &'static str>> + Send;

    fn file_get(
        &self,
        file_id: &uuid::Uuid
    ) -> impl Future<Output = Result<File, &'static str>> + Send;
}