#![allow(clippy::needless_return)]

use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct Person {
	pub first_name: String,
	pub middle_name: String,
	pub last_name: String,
	pub prefix: String,
	pub suffix: String,
	pub gender: i16,
}

pub trait CrmProvider {

	fn person_save(
		&self,
		tenant_id: &uuid::Uuid,
		person: Person
	) -> impl Future<Output = Result<(), &'static str>> + Send;
}
