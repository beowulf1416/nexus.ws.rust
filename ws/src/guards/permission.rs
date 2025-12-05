use tracing::{
    info,
    error,
    debug
};

use actix_web::{
    http, 
    web, 
    guard::{
        Guard,
        GuardContext
    },
    HttpResponse, 
    Responder
};



#[derive(Debug)]
pub struct Permission {
    permission: String
}


impl Permission {

    pub fn new(name: &str) -> Self {
        return Self {
            permission: String::from(name)
        };
    }
}

impl Guard for Permission {

    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        return false;
    }
}