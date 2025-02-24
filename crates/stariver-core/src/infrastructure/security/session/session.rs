use anyhow::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
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

#[cfg(test)]
mod tests {
    use hmac::{Hmac, Mac};
    use redis::Commands;
    use sha2::Sha256;
    use std::time::Instant;

    pub fn test_redis() {
        // connect to redis
        let client =
            redis::Client::open("redis://127.0.0.1/").expect("failed to create redis client");
        let mut con = client
            .get_connection()
            .expect("failed to get redis connection");
        let _: () = con.set("my_key", 42).expect("failed to set value");

        let start = Instant::now();
        let val: i64 = con.get("my_key").expect("failed to get my_key");
        println!("{}", val);
        let end = Instant::now();
        println!("代码执行时间: {:?} 微秒", (end - start).as_micros());
    }

    #[test]
    pub fn test_hmac() {
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
            .expect("HMAC can take key of any size");
        mac.update(b"input message input message nput message input message nput message input message nput message input message nput message input message nput message input message nput message input message nput message input message");

        // `result` has type `CtOutput` which is a thin wrapper around array of
        // bytes for providing constant time equality check
        let result = mac.finalize();
        // To get underlying array use `into_bytes`, but be careful, since
        // incorrect use of the code value may permit timing attacks which defeats
        // the security provided by the `CtOutput`
        let code_bytes = result.into_bytes();
        let start = Instant::now();
        let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
            .expect("HMAC can take key of any size");

        mac.update(b"input message input message nput message input message nput message input message nput message input message nput message input message nput message input message nput message input message nput message input message");

        // `verify_slice` will return `Ok(())` if code is correct, `Err(MacError)` otherwise
        mac.verify_slice(&code_bytes[..])
            .expect("HMAC verification failed");
        let end = Instant::now();
        println!("代码执行时间: {:?} 微秒", (end - start).as_micros());
    }
}
