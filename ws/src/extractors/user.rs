use serde::Deserialize;
use tracing::{
    info,
    debug,
    error
};
use std::sync::Arc;
use std::fmt::Display;
use std::pin::Pin;
use futures::{
    future::{
        ok,
        err,
        Ready
    }
};

use actix_web::{
    web,
    FromRequest,
    ResponseError
};
use actix_http::{
    header,
    Method,
    HttpMessage,
    StatusCode
};
use serde::Serialize;


use users_provider::UsersProvider;
use crate::extractors;




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
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_http::Payload) -> Self::Future {
        if req.method() == Method::POST {
            // check if already have value
            if let Some(u) = req.extensions().get::<extractors::user::User>() {
                let cloned = u.clone();
                return Box::pin(async move{
                    return Ok(cloned);
                });
            }
        
            // check for token
            if let Some(header_value) = req.headers().get(header::AUTHORIZATION) 
                && let Ok(token_value) = header_value.to_str() {

                let pattern = regex::Regex::new(r"(?i)bearer").expect("incorrect regex pattern to retrieve bearer authentication");
                let token = pattern.replace(token_value, "").to_string();
                let token = token.trim();

                let mut user_id = uuid::Uuid::nil();

                // validate token
                if let Some(tg) = req.app_data::<web::Data<Arc<token::TokenGenerator>>>() {
                    let claim = tg.claim(&token);
                    if !claim.is_empty() {
                        user_id = claim.user_id;
                    }
                }

                if !user_id.is_nil() && let Some(dp_ref) = req.app_data::<web::Data<Arc<database_provider::DatabaseProvider>>>() {
                    let dp = dp_ref.get_ref().clone();

                    return Box::pin(async move {
                        debug!("should only happen once");
                        
                        let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

                        if let Ok(user) = up.fetch_by_id(&user_id).await {
                            let u = extractors::user::User::new(
                                &user_id,
                                // user_name
                                &user.email
                            );

                            // let cloned_1 = u.clone();
                            // req.extensions_mut().insert(cloned_1);

                            // let cloned_2 = u.clone();
                            return Ok(u);
                        } else {
                            error!("unable to extract user");
                            return Ok(User::anonymous());
                        }
                    });
                }
            }
        }

        error!("unable to extract user");
        return Box::pin(async move {
            return Ok(User::anonymous());
        });
    }
}


