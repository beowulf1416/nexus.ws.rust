pub mod session;
pub mod user;


use tracing::{
    info
};

use serde::{
    Serialize,
    Deserialize
};
use serde_json::Value;

use actix_web::{
    HttpResponse, 
    Responder
};



#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    success: bool,
    message: &'static str,
    data: Option<Value>
}


impl ApiResponse {
    pub fn new(
        success: bool,
        message: &'static str,
        data: Option<Value>
    ) -> Self {
        return Self {
            success,
            message,
            data: data.clone()
        };
    }

    pub fn ok(message: &'static str) -> Self {
        return Self {
            success: true,
            message,
            data: None
        };
    }

    pub fn error(message: &'static str) -> Self {
        return Self {
            success: false,
            message,
            data: None
        };
    }
}


pub async fn default_option_response() -> impl Responder {
    info!("default_option_response");
    return HttpResponse::Ok()
        // .append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .finish();
}