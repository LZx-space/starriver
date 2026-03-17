use serde::{Deserialize, Serialize};
use time::UtcDateTime;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RegisteredClaims {
    pub iss: Option<String>, // Optional. Issuer
    pub sub: Option<String>, // Optional. Subject (whom token refers to)
    pub aud: Option<String>, // Optional. Audience
    pub exp: Option<i64>,    // Optional Expiration time (as UTC timestamp)
    pub nbf: Option<i64>,    // Optional. Not Before (as UTC timestamp)
    pub iat: Option<i64>,    // Optional. Issued at (as UTC timestamp)
    pub jti: Option<String>, // Optional. JWT ID
}

impl RegisteredClaims {
    pub fn with_sub(mut self, sub: String) -> Self {
        self.sub = Some(sub);
        self
    }

    pub fn with_exp(mut self, exp: UtcDateTime) -> Self {
        let exp = exp.unix_timestamp();
        self.exp = Some(exp);
        self
    }

    pub fn with_nbf(mut self, nbf: UtcDateTime) -> Self {
        let nbf = nbf.unix_timestamp();
        self.nbf = Some(nbf);
        self
    }

    pub fn with_iat(mut self, iat: UtcDateTime) -> Self {
        let iat = iat.unix_timestamp();
        self.iat = Some(iat);
        self
    }
}

#[cfg(test)]
mod test {
    use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

    use super::*;

    #[test]
    fn demo1() {
        let claims = RegisteredClaims::default().with_exp(UtcDateTime::now());
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        );
        match token {
            Ok(jwt) => {
                println!("encode jwt: {}", jwt);
                let mut validation = Validation::default();
                validation.set_audience(&vec!["LZx"]);
                let decode = decode::<RegisteredClaims>(
                    &jwt,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &validation,
                );
                match decode {
                    Ok(t) => {
                        println!("decode = {:?}", t);
                    }
                    Err(err) => {
                        println!("decode err: {}", err);
                    }
                }
            }
            Err(err) => {
                panic!("encode err: {}", err)
            }
        }
    }
}
