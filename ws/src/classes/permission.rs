use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Permission {
    id: i32,
    name: String
}


impl Permission {

    pub fn new(
        permission_id: &i32,
        name: &String
    ) -> Self {
        return Self {
            id: permission_id.clone(),
            name: name.clone()
        };
    }

    pub fn id(&self) -> i32 {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}