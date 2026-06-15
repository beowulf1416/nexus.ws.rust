#![allow(clippy::needless_return)]

use std::str::FromStr;

use tracing::{
    info,
    error,
    debug
};

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthData {
	pub user_id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub email: String,
    pub username: String,
}

impl AuthData {

    pub fn default() -> Self {
        Self {
            user_id: uuid::Uuid::nil(),
            tenant_id: uuid::Uuid::nil(),
            email: String::new(),
            username: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.user_id == uuid::Uuid::nil()
            && self.tenant_id == uuid::Uuid::nil()
            && self.email.is_empty()
            && self.username.is_empty();
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claim {
	pub sub: String,
	pub client_id: String,
	pub email: String,
	pub preferred_username: String,

    pub iat: usize,
    pub exp: usize,
    pub nbf: usize
}



#[derive(Debug, Clone)]
pub struct TokenGenerator {
    secret: String
}


impl TokenGenerator {

    pub fn new(
        secret: &str
    ) -> Self {
        return Self {
            secret: String::from(secret)
        };
    }

    pub fn generate(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid,
        user_name: &str,
        email: &str
    ) -> Result<String, &'static str> {
        info!("generate");

        let now = chrono::Utc::now();
        let expiry = now.checked_add_signed(chrono::TimeDelta::hours(1)).unwrap();

        let header = Header {
            alg: Algorithm::HS512,
            kid: Some(String::from("todo")),
            ..Default::default()
        };

        let claims = Claim {
            sub: user_id.to_string(),
            client_id: tenant_id.to_string(),
            email: String::from(email),
            preferred_username: String::from(user_name),
            iat: now.timestamp() as usize,
            exp: expiry.timestamp() as usize,
            nbf: now.timestamp() as usize
        };

        let token = match encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ) {
            Err(e) => {
                error!("unable to encode token: {}", e);
                return Err("unable to encode token");
            }
            Ok(result) => {
                result
            }
        };

        return Ok(token);
    }


    pub fn parse_token(&self, token: &str) -> Result<AuthData, &'static str> {
        info!("parse_tokens");
        debug!("token: [{}]", token);


        let tokens = match decode::<Claim>(
            &token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        ) {
            Err(e) => {
                error!("unable to decode token: {}", e);
                return Err("unable to decode token");
            }
            Ok(tokens) => tokens,
        };

        let claim = tokens.claims;
        let user_id = match uuid::Uuid::from_str(claim.sub.as_str()) {
            Err(e) => {
                error!("unable to parse user_id: {}", e);
                uuid::Uuid::nil()
            }
            Ok(user_id) => user_id,
        };
        let tenant_id = match uuid::Uuid::from_str(claim.client_id.as_str()) {
            Err(e) => {
                error!("unable to parse client_id: {}", e);
                uuid::Uuid::nil()
            }
            Ok(client_id) => client_id,
        };

        return Ok(AuthData {
            user_id: user_id,
            tenant_id: tenant_id,
            email: claim.email,
            username: claim.preferred_username,
        });
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
