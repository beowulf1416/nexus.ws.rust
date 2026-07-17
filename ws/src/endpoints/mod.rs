pub mod common;
pub mod file;
pub mod permissions;
pub mod session;
pub mod user;
// pub mod documents;
pub mod acctg;
pub mod admin;
pub mod crm;
pub mod inventory;

use actix_http::header;
use tracing::info;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use actix_web::{HttpResponse, Responder, http};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    success: bool,
    message: &'static str,
    data: Option<Value>,
}

impl ApiResponse {
    pub fn new(success: bool, message: &'static str, data: Option<Value>) -> Self {
        return Self {
            success,
            message,
            data: data.clone(),
        };
    }

    pub fn ok(message: &'static str) -> Self {
        return Self {
            success: true,
            message,
            data: None,
        };
    }

    pub fn error(message: &'static str) -> Self {
        return Self {
            success: false,
            message,
            data: None,
        };
    }
}

pub async fn default_option_response() -> impl Responder {
    info!("default_option_response");
    return HttpResponse::Ok()
        // .append_header((http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        // .append_header((http::header::ACCESS_CONTROL_ALLOW_METHODS, "OPTIONS, POST"))
        // .append_header((
        //     http::header::ACCESS_CONTROL_ALLOW_HEADERS,
        //     "Content-Type, Authorization",
        // ))
        .finish();
}
