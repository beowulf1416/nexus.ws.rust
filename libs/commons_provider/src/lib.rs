use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code_2: String,
    pub code_3: String,
}

#[derive(Serialize)]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub unit_text: String,
    pub symbol: Option<String>,
}

#[derive(Serialize)]
pub struct Gender {
    pub id: i16,
    pub name: String,
}

#[derive(Serialize)]
pub struct Dimension {
    pub id: i16,
    pub name: String,
}

#[derive(Serialize)]
pub struct Uom {
    pub id: i32,
    pub dimension_id: i16,
    pub name: String,
    pub symbol: Option<String>,
}

pub trait CommonsProvider {
    fn fetch_countries(&self) -> impl Future<Output = Result<Vec<Country>, &'static str>> + Send;

    fn fetch_currencies(&self) -> impl Future<Output = Result<Vec<Currency>, &'static str>> + Send;

    fn fetch_genders(&self) -> impl Future<Output = Result<Vec<Gender>, &'static str>> + Send;

    fn fetch_dimensions(&self)
    -> impl Future<Output = Result<Vec<Dimension>, &'static str>> + Send;

    fn fetch_uoms(&self) -> impl Future<Output = Result<Vec<Uom>, &'static str>> + Send;

    fn fetch_uoms_by_dimension_id(
        &self,
        dimension_id: &i16,
    ) -> impl Future<Output = Result<Vec<Uom>, &'static str>> + Send;
}
