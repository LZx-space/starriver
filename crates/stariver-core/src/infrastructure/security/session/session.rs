use anyhow::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub struct Session {
    username: String,
}

impl TryFrom<JWT> for Session {
    type Error = Error;

    fn try_from(value: JWT) -> Result<Self, Self::Error> {
        decode::<Claims>(
            &value.0,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .map(|data| Session {
            username: data.claims.sub,
        })
        .map_err(|e| Error::msg(e.to_string()))
    }
}

impl TryInto<JWT> for Session {
    type Error = Error;

    fn try_into(self) -> Result<JWT, Self::Error> {
        let claims = Claims {
            aud: "S".to_string(),
            exp: 0,
            iat: 0,
            iss: "".to_string(),
            nbf: 0,
            sub: self.username,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .map(|e| JWT(e))
        .map_err(|e| Error::msg(e.to_string()))
    }
}

pub struct JWT(String);

impl TryFrom<String> for JWT {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let count = value.split(".").count();
        if count != 2 {
            return Err(Error::msg("bad format"));
        }
        let len = value.len();
        if len > 256 {
            return Err(Error::msg("too long"));
        }
        Ok(JWT(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}
