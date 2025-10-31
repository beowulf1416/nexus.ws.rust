use serde::Deserialize;
use tracing::{
    info,
    debug,
    error
};
use std::fmt::Display;
use futures::{
    future::{
        ok,
        err,
        Ready
    }
};

use actix_web::{
    FromRequest,
    ResponseError
};
use actix_http::{
    HttpMessage,
    StatusCode
};
use serde::Serialize;




#[derive(Debug)]
pub enum UserError {
    InternalServerError
}


impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserError::InternalServerError => write!(f, "internal server error")
        }
    }
}

impl ResponseError for UserError {

    fn status_code(&self) -> actix_http::StatusCode {
        return StatusCode::INTERNAL_SERVER_ERROR; 
    }
}




#[derive(Debug, Clone, Serialize)]
pub struct User {
    user_id: uuid::Uuid,
    user_name: String
}


impl User {

    pub fn new(
        user_id: &uuid::Uuid,
        user_name: &str
    ) -> Self {
        return Self {
            user_id: user_id.clone(),
            user_name: String::from(user_name)
        };
    }

    pub fn anonymous() -> Self {
        return Self {
            user_id: uuid::Uuid::nil(),
            user_name: String::from("")
        };
    }

    pub fn is_anonymous(&self) -> bool {
        return self.user_id.is_nil();
    }

    pub fn user_id(&self) -> uuid::Uuid {
        return self.user_id.clone();
    }

    pub fn user_name(&self) -> String {
        return self.user_name.clone();
    }
}



impl FromRequest for User {
    type Error = UserError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_http::Payload) -> Self::Future {
        if let Some(user) = req.extensions().get::<User>() {
            return ok(user.clone());
        }

        return ok(User::anonymous());
    }
}


