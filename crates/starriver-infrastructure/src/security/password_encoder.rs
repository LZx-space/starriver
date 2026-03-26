use anyhow::Error;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHashString, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

pub trait PasswordEncoder {
    fn encode(&self, password: &str) -> core::result::Result<String, Error>;
    fn verify(&self, raw_password: &str, encode_password: &str) -> core::result::Result<(), Error>;
}

#[derive(Default)]
pub struct Argon2PasswordEncoder {
    argon2: Argon2<'static>,
}

impl PasswordEncoder for Argon2PasswordEncoder {
    fn encode(&self, password: &str) -> core::result::Result<String, Error> {
        let salt_string = SaltString::generate(&mut OsRng);
        let hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| Error::msg(e.to_string()))?;
        Ok(hash.serialize().to_string())
    }

    fn verify(&self, raw_password: &str, encode_password: &str) -> core::result::Result<(), Error> {
        let password_hash_str =
            PasswordHashString::new(encode_password).map_err(|e| Error::msg(e.to_string()))?;
        let password_hash = password_hash_str.password_hash();
        self.argon2
            .verify_password(raw_password.as_bytes(), &password_hash)
            .map_err(|e| Error::msg(e.to_string()))
    }
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher, PasswordVerifier};

    #[test]
    pub fn argon2_test() {
        let start = SystemTime::now();
        let argon2 = Argon2::default();
        let pwd_bytes = "123".as_bytes();
        let salt_string = SaltString::encode_b64("starriver".as_bytes()).expect("啊啊啊");
        let hash = argon2
            .hash_password(pwd_bytes, &salt_string)
            .expect("啵啵啵");
        let duration = SystemTime::now().duration_since(start).expect("啊啊啊");
        println!("{:?}", duration);
        let result = argon2.verify_password(pwd_bytes, &hash);
        println!("{:?}", result);
        let hash_string = hash.serialize();
        let hash_str = hash_string.as_str();
        println!("{}", hash_str);
    }
}
