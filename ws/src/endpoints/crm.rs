use tracing::{
    info,
    error,
    debug
};

use std::sync::Arc;
use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

// use http::header::AUTHORIZATION;
use actix_web::{
    guard,
    http,
    web,
    HttpResponse,
    Responder
};

use crate::{
    classes::{
        user,
        // tenant,
        // permission
    },
    endpoints::{
        ApiResponse,
        default_option_response
    }
};

use crm_provider::CrmProvider;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("person/save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(person_save_post))
        )
        .service(
            web::resource("business/save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(business_save_post))
        )
    ;
}


#[derive(Debug, Serialize, Deserialize)]
struct PersonSavePostData {
	tenant_id: uuid::Uuid,
	person_id: uuid::Uuid,

	first_name: String,
    middle_name: String,
    last_name: String,
    prefix: String,
    suffix: String,
    gender: i16,
}

async fn person_save_post(
	dp: web::Data<Arc<database_provider::DatabaseProvider>>,
	user: user::User,
	params: web::Json<PersonSavePostData>,
) -> impl Responder {
	info!("person_save_post");

	let crm_provider = crm_provider_postgres::PostgresCrmProvider::new(&dp);

	let person = crm_provider::Person {
		people_id: params.person_id.clone(),
		active: true,
		created: chrono::Utc::now(),
		first_name: params.first_name.clone(),
		middle_name: params.middle_name.clone(),
		last_name: params.last_name.clone(),
		prefix: params.prefix.clone(),
		suffix: params.suffix.clone(),
		gender: params.gender.clone(),
	};

	match crm_provider.person_save(
		&params.tenant_id,
		&person,
	).await {
		Err(e) => {
			error!("unable to save person record: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to save person record"));
		}
		Ok(_) => {
			return HttpResponse::Ok().json(
				ApiResponse::ok("successfully saved person record")
			);
		}
	}
}


#[derive(Debug, Serialize, Deserialize)]
struct BusinessSavePostData {
	tenant_id: uuid::Uuid,
	business_id: uuid::Uuid,
	name: String,
	description: String,
}

async fn business_save_post(
	dp: web::Data<Arc<database_provider::DatabaseProvider>>,
	user: user::User,
	params: web::Json<BusinessSavePostData>,
) -> impl Responder {
	info!("business_save_post");

		let crm_provider = crm_provider_postgres::PostgresCrmProvider::new(&dp);

		let business = crm_provider::Business {
			business_id: params.business_id.clone(),
			name: params.name.clone(),
			description: params.description.clone(),
		};

		match crm_provider.business_save(
			&params.tenant_id,
			&business,
		).await {
			Err(e) => {
				error!("unable to save business record: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to save business record"));
			}
			Ok(_) => {
				return HttpResponse::Ok().json(
					ApiResponse::ok("successfully saved business record")
				);
			}
		}
}
