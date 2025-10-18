use actix_http::header::HeaderValue;
use log::{
    error,
    info,
    warn,
    debug
};

use actix_web::{
    body::MessageBody, dev::{
        ServiceRequest,
        ServiceResponse
    }, http::{
        header, Method
    }, middleware::{
        self,
        Next
    }, web, Error, HttpMessage
};


pub async fn cors_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    debug!("method: {:?}", req.method());

    // invoke the wrapped middleware or service
    let mut res = next.call(req).await?;

    // post-processing
    res.headers_mut().append(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    res.headers_mut().append(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("POST, OPTIONS"));
    res.headers_mut().append(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("content-type, authorization"));
    res.headers_mut().append(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("true"));
    res.headers_mut().append(header::ACCESS_CONTROL_EXPOSE_HEADERS, HeaderValue::from_static("authorization"));

    // debug!("post: cors_middleware");

    Ok(res)
}