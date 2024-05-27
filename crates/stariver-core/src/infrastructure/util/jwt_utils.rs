#[cfg(test)]
mod test {
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        // aud: String, // Optional. Audience
        exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
        iat: usize, // Optional. Issued at (as UTC timestamp)
        iss: String, // Optional. Issuer
        nbf: usize, // Optional. Not Before (as UTC timestamp)
        sub: String, // Optional. Subject (whom token refers to)
    }

    #[test]
    fn demo1() {
        let claims = Claims {
            // aud: "LZx".to_string(),
            exp: 2_000_000_000,
            iat: 1_661_231_234,
            iss: "LZx".to_string(),
            nbf: 1_669_231_234,
            sub: "token".to_string(),
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        );
        match token {
            Ok(jsonwebtoken) => {
                println!("{}", jsonwebtoken);
                let decode = decode::<Claims>(
                    &jsonwebtoken,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::default(),
                );
                match decode {
                    Ok(t) => {
                        println!("decode = {:?}", t);
                    }
                    Err(err) => {
                        println!("decode err {}", err);
                    }
                }
            }
            Err(err) => {
                panic!("encode err {}", err)
            }
        }
    }
}
