use tracing::{
    info
};
use std::sync::Arc;
use actix_web::{
    web,
    HttpMessage,
    body::MessageBody,
    dev::{
        ServiceRequest,
        ServiceResponse
    },
    middleware::Next,
    Error
};
use actix_http::{
    Method,
    header
};


use users_provider::UsersProvider;

use crate::extractors;


pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    info!("auth_middleware");

    if req.method() == Method::POST && let Some(header_value) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(token_value) = header_value.to_str() {
            let pattern = regex::Regex::new(r"(?i)bearer").unwrap();
            let token = pattern.replace(token_value, "").to_string();

            let mut user_id = uuid::Uuid::nil();

            if let Some(tg) = req.app_data::<web::Data<Arc<token::TokenGenerator>>>() {
                let claim = tg.claim(&token);
                if !claim.is_empty() {
                    user_id = claim.user_id;
                }
            }

            if let Some(dp_ref) = req.app_data::<web::Data<Arc<database_provider::DatabaseProvider>>>() {
                let dp = dp_ref.get_ref();
                let up = users_provider_postgres::PostgresUsersProvider::new(&dp);

                if let Ok(user) = up.fetch_by_id(&user_id).await {
                    let u = extractors::user::User::new(
                        &user_id,
                        // user_name
                        &"//todo"
                    );

                    req.extensions_mut().insert(u);
                }
            }
        }
    }

    let res = next.call(req).await?;

    return Ok(res);
}