use tracing::{
    info,
    error
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
        sid: &uuid::Uuid,
        tenant_id: &uuid::Uuid,
        uname: &str
    ) -> Result<String, &'static str> {
        info!("generate");

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        
        let now = chrono::Utc::now();
        let expiry = now.checked_add_signed(chrono::TimeDelta::hours(1)).unwrap();

        claims.insert("iat", now.timestamp().to_string());
        claims.insert("exp", expiry.timestamp().to_string());

        claims.insert("sid", sid.to_string());
        claims.insert("client_id", tenant_id.to_string());
        claims.insert("preferred_username", uname.to_string());

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

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        
        let result: Result<BTreeMap<String, String>, error::Error> = token.verify_with_key(&key);
        if let Err(e) = result {
            error!("unable to validate token: {}" ,e);
            return false;
        };

        return true;
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
