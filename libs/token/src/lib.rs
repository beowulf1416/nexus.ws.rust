#![allow(clippy::needless_return)]

use tracing::{
    info,
    error,
    debug
};

use std::collections::BTreeMap;

use hmac::{
    Hmac,
    Mac
};
use sha2::Sha256;

use jwt::{
    SignWithKey,
    VerifyWithKey,
    error
};



#[derive(Debug, Clone)]
pub struct Claim {
    pub user_id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub user_name: String,
    pub email: String
}


impl Claim {
    pub fn new(
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid,
        user_name: &str,
        email: &str
    ) -> Self {
        return Self {
            user_id: *user_id,
            tenant_id: tenant_id.clone(),
            user_name: String::from(user_name),
            email: String::from(email)
        };
    }

    pub fn empty() -> Self {
        return Self {
            user_id: uuid::Uuid::nil(),
            tenant_id: uuid::Uuid::nil(),
            user_name: String::from(""),
            email: String::from("")
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.user_id.is_nil();
    }
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
        claim: &Claim
    ) -> Result<String, &'static str> {
        info!("generate");

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        
        let now = chrono::Utc::now();
        let expiry = now.checked_add_signed(chrono::TimeDelta::hours(1)).unwrap();

        claims.insert("iat", now.timestamp().to_string());
        claims.insert("exp", expiry.timestamp().to_string());

        claims.insert("sid", claim.user_id.to_string());
        claims.insert("client_id", claim.tenant_id.to_string());
        claims.insert("preferred_username", claim.user_name.to_string());
        claims.insert("email", claim.email.to_string());

        return match claims.sign_with_key(&key) {
            Err(e) => {
                error!("unable to sign claims: {}", e);
                Err("unable to sign claims")
            }
            Ok(result) => {
                Ok(result)
            }
        };
    }


    pub fn validate(
        &self,
        token: &str
    ) -> bool {
        info!("validate");
        debug!("token: [{}]", token);

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        
        let result: Result<BTreeMap<String, String>, error::Error> = token.verify_with_key(&key);
        if let Err(e) = result {
            error!("unable to validate token: [{}]" ,e);
            return false;
        };

        return true;
    }

    pub fn claim(&self, token: &str) -> Claim {
        info!("claim");
        debug!("token: [{}]", token);

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let result: Result<BTreeMap<String, String>, error::Error> = token.verify_with_key(&key);
        match result {
            Err(e) => {
                error!("unable to verify token: [{}]", e);
                return Claim::empty();
            }
            Ok(claims) => {
                let user_id = match claims.get("sid") {
                    None => uuid::Uuid::nil(),
                    Some(id) => {
                        match uuid::Uuid::parse_str(id) {
                            Err(e) => {
                                error!("invalid user id");
                                uuid::Uuid::nil()
                            }
                            Ok(uid) => uid
                        }
                    }
                };

                let tenant_id = match claims.get("client_id") {
                    None => uuid::Uuid::nil(),
                    Some(id) => 
                        match uuid::Uuid::parse_str(id) {
                            Err(e) => {
                                error!("invalid user id");
                                uuid::Uuid::nil()
                            }
                            Ok(tid) => tid
                        }
                };

                let user_name = match claims.get("uname") {
                    None => String::from(""),
                    Some(user_name) => user_name.to_string()
                };

                let email = match claims.get("email") {
                    None => String::from(""),
                    Some(email) => email.to_string()
                };

                return Claim::new(
                    &user_id,
                    &tenant_id,
                    user_name.as_str(),
                    email.as_str()
                );
            }
        }
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
