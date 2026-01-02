use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code_2: String,
    pub code_3: String
}


#[derive(Serialize)]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub unit_text: String,
    pub symbol: Option<String>
}


#[derive(Serialize)]
pub struct Gender {
    pub id: i16,
    pub name: String
}


pub trait CommonsProvider {

    fn fetch_countries(&self) -> impl Future<Output = Result<Vec<Country>, &'static str>> + Send;

    fn fetch_currencies(&self) -> impl Future<Output = Result<Vec<Currency>, &'static str>> + Send;

    fn fetch_genders(&self) -> impl Future<Output = Result<Vec<Gender>, &'static str>> + Send;
}