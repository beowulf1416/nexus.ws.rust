// https://docs.rs/actix-web/latest/actix_web/middleware/index.html

use tracing::{
    info
};

use std::{
    future::{
        ready,
        Ready,
        Future
    },
    pin::Pin
};

use actix_web::{
    body::MessageBody,
    dev::{
        Service,
        ServiceRequest,
        ServiceResponse,
        Transform, forward_ready
    },
    error::Error
};


pub struct Permission {
    permission: String
}


impl Permission {

    pub fn new(
        permission: &str
    ) -> Self {
        return Self {
            permission: String::from(permission)
        };
    }
}


pub struct PermissionsMiddleware<S> {
    service: S
}



type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl <S, B> Service<ServiceRequest> for PermissionsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;
    
    // fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
    //     todo!()
    // }

    forward_ready!(service);
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        todo!()
    }
}



impl<S, B> Transform<S, ServiceRequest> for Permission 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{

    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PermissionsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        todo!()
    }
}