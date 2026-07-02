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
            web::resource("partner/save")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(partner_save_post))
        )
        // .service(
        //     web::resource("business/save")
        //         .route(web::method(http::Method::OPTIONS).to(default_option_response))
        //         .route(web::post().guard(guard::Header("content-type", "application/json")).to(business_save_post))
        // )
        .service(
            web::resource("partners/fetch")
                .route(web::method(http::Method::OPTIONS).to(default_option_response))
                .route(web::post().guard(guard::Header("content-type", "application/json")).to(partners_fetch_post))
        )
    ;
}


#[derive(Debug, Serialize, Deserialize)]
struct PartnerSavePostData {
	tenant_id: uuid::Uuid,
	partner_id: uuid::Uuid,

	business_name: String,
	description: String,

	first_name: String,
    middle_name: String,
    last_name: String,
    prefix: String,
    suffix: String,
    gender: i16,
}

async fn partner_save_post(
	dp: web::Data<Arc<database_provider::DatabaseProvider>>,
	user: user::User,
	params: web::Json<PartnerSavePostData>,
) -> impl Responder {
	info!("partner_save_post");

	let crm_provider = crm_provider_postgres::PostgresCrmProvider::new(&dp);

	let partner = crm_provider::Partner {
		partner_id: params.partner_id.clone(),

		active: true,
		created: chrono::Utc::now(),

		business_name: params.business_name.clone(),
		description: params.description.clone(),

		first_name: params.first_name.clone(),
		middle_name: params.middle_name.clone(),
		last_name: params.last_name.clone(),
		prefix: params.prefix.clone(),
		suffix: params.suffix.clone(),
		// gender: params.gender.clone(),
	};

	match crm_provider.partner_save(
		&params.tenant_id,
		&partner,
	).await {
		Err(e) => {
			error!("unable to save partner record: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to save partner record"));
		}
		Ok(_) => {
			return HttpResponse::Ok().json(
				ApiResponse::ok("successfully saved partner record")
			);
		}
	}
}


// #[derive(Debug, Serialize, Deserialize)]
// struct BusinessSavePostData {
// 	tenant_id: uuid::Uuid,
// 	business_id: uuid::Uuid,
// 	name: String,
// 	description: String,
// }

// async fn business_save_post(
// 	dp: web::Data<Arc<database_provider::DatabaseProvider>>,
// 	user: user::User,
// 	params: web::Json<BusinessSavePostData>,
// ) -> impl Responder {
// 	info!("business_save_post");

// 	let crm_provider = crm_provider_postgres::PostgresCrmProvider::new(&dp);

// 	let business = crm_provider::Business {
// 		business_id: params.business_id.clone(),
// 		name: params.name.clone(),
// 		description: params.description.clone(),
// 	};

// 	match crm_provider.business_save(
// 		&params.tenant_id,
// 		&business,
// 	).await {
// 		Err(e) => {
// 			error!("unable to save business record: {}", e);
//         return HttpResponse::InternalServerError()
//             .json(ApiResponse::error("unable to save business record"));
// 		}
// 		Ok(_) => {
// 			return HttpResponse::Ok().json(
// 				ApiResponse::ok("successfully saved business record")
// 			);
// 		}
// 	}
// }


#[derive(Debug, Serialize, Deserialize)]
struct PartnersFetchPostData {
	tenant_id: uuid::Uuid,
	filter: String
}

async fn partners_fetch_post(
	dp: web::Data<Arc<database_provider::DatabaseProvider>>,
	user: user::User,
	params: web::Json<PartnersFetchPostData>,
) -> impl Responder {
	info!("partners_fetch_post");

	let crm_provider = crm_provider_postgres::PostgresCrmProvider::new(&dp);

    let filter = format!("%{}%", params.filter);

    match crm_provider.partners_fetch(
        &params.tenant_id,
        &filter
    ).await {
        Err(e) => {
            error!("unable to fetch permissions: {}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error("unable to fetch permissions"));
        }
        Ok(partners) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully fetched permissions",
                    Some(json!({
                        "partners": partners
                    })
                )));
        }
    }

}
