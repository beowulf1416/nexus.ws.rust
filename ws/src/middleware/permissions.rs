// https://docs.rs/actix-web/latest/actix_web/middleware/index.html

use tracing::{
    info,
    debug,
    error
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
    HttpMessage,
    HttpRequest, 
    HttpResponse, 
    body::{
        MessageBody
    }, 
    dev::{
        Service,
        ServiceRequest,
        ServiceResponse,
        Transform, 
        forward_ready
    }, 
    error::Error
};
use actix_http::{Method};


use crate::{classes::user, endpoints::ApiResponse};



#[derive(Debug, Clone)]
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
    service: S,
    permission: Permission
}



type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl <S> Service<ServiceRequest> for PermissionsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
    // B: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;
    
    // fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
    //     todo!()
    // }

    forward_ready!(service);
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("call");

        let user = {
            let binding = req.extensions();
            match binding.get::<user::User>() {
                None => {
                    user::User::anonymous().clone()
                }
                Some(u) => {
                    u.clone()
                }
            }
        };

        // debug!("user: {:?}", user);

        let requested_permission = self.permission.permission.clone();

        // if the endpoint is protected by a permission
        if !requested_permission.is_empty()
            && req.method() == Method::POST
        {
            if user.is_anonymous() {
                debug!("user is anonymous");
                return Box::pin( async move {
                    let res = HttpResponse::Unauthorized()
                        .json(ApiResponse::error("user is not authenticated"))
                    ;

                    return Ok(req.into_response(res));
                });
            } else {
                // check if user has permission
                let allowed = user.permissions().iter().any(|p| {
                    return p.name() == requested_permission;
                });

                if !allowed {
                    debug!("user is missing permission: {}", requested_permission);
                    return Box::pin( async move {
                        let res = HttpResponse::Forbidden()
                            .json(ApiResponse::error("user is not allowed"))
                        ;

                        return Ok(req.into_response(res));
                    });
                }
            }
        }


        let fut = self.service.call(req);

        return Box::pin(async move {
            let res = fut.await?;
            info!("result");
            return Ok(res);
        });
    }
}



impl<S> Transform<S, ServiceRequest> for Permission 
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
    // B: 'static,
{

    type Response = ServiceResponse;
    type Error = Error;
    type Transform = PermissionsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(PermissionsMiddleware { 
            service,
            permission: self.clone() 
        }));
    }
}