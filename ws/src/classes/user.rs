use tracing::{
    info,
    error,
    debug
};

use std::fmt::Display;
use std::pin::Pin;
use actix_web::{
    web,
    FromRequest,
    ResponseError,
    HttpMessage
};
use actix_http::{
    StatusCode,
    Method
};

use crate::classes::{
    permission,
    user,
    tenant
};



#[derive(Debug, Clone)]
pub struct User {
    user_id: uuid::Uuid,
    tenant: tenant::Tenant,
    name: String,
    email: String,
    tenants: Vec<tenant::Tenant>,
    permissions: Vec<permission::Permission>
}



#[derive(Debug)]
pub enum UserError {
    InternalServerError
}



impl User {

    pub fn new(
        user_id: &uuid::Uuid,
        tenant: &tenant::Tenant,
        name: &str,
        email: &str,
        tenants: &Vec<tenant::Tenant>,
        permissions: &Vec<permission::Permission>
    ) -> Self {
        return Self {
            user_id: user_id.clone(),
            tenant: tenant.clone(),
            name: String::from(name),
            email: String::from(email),
            tenants: tenants.clone(),
            permissions: permissions.clone()
        };
    }

    pub fn anonymous() -> Self {
        return Self {
            user_id: uuid::Uuid::nil(),
            tenant: tenant::Tenant::default(),
            name: String::from(""),
            email: String::from(""),
            tenants: vec![],
            permissions: vec![]
        };
    }

    pub fn user_id(&self) -> uuid::Uuid {
        return self.user_id;
    }

    pub fn tenant(&self) -> tenant::Tenant {
        return self.tenant.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn email(&self) -> String {
        return self.email.clone();
    }

    pub fn is_anonymous(&self) -> bool {
        return self.user_id.is_nil();
    }

    pub fn is_authenticated(&self) -> bool {
        return !self.user_id.is_nil();
    }

    pub fn tenants(&self) -> Vec<tenant::Tenant> {
        return self.tenants.clone();
    }

    pub fn permissions(&self) -> Vec<permission::Permission> {
        return self.permissions.clone();
    }
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


impl FromRequest for User {
    type Error = UserError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_http::Payload) -> Self::Future {
        info!("from_request");

        if req.method() == Method::POST {
            // check if already have value
            if let Some(u) = req.extensions().get::<user::User>() {
                let cloned = u.clone();
                return Box::pin(async move{
                    return Ok(cloned);
                });
            }
        }

        return Box::pin(async move {
            return Ok(User::anonymous());
        });
    }
}