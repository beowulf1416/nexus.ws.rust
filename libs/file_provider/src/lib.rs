use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub folder_id: uuid::Uuid,
    pub name: String
}


impl Folder {

    pub fn new (
        folder_id: uuid::Uuid,
        name: String 
    ) -> Self {
        return Folder {
            folder_id,
            name
        };
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub file_id: uuid::Uuid,
    pub name: String
}


impl File {

    pub fn new(
        file_id: uuid::Uuid,
        name: String 
    ) -> Self {
        return File {
            file_id,
            name
        };
    }
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

    fn folder_list_folders(
        &self,
        folder_id: &uuid::Uuid
    ) -> impl Future<Output = Result<Vec<Folder>, &'static str>> + Send;

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