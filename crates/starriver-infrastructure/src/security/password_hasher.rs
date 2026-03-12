use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHashString, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash};
use password_hash::errors::Result;

pub fn from_hashed_password(hashed_password: &str) -> Result<PasswordHashString> {
    PasswordHashString::new(hashed_password)
}

pub fn hash_password(password: &str) -> Result<PasswordHashString> {
    let argon2 = Argon2::default();
    let salt_string = SaltString::generate(&mut OsRng);
    let hash = argon2.hash_password(password.as_bytes(), &salt_string)?;
    Ok(hash.serialize())
}

pub fn verify_password(password: &str, password_hash_string: &PasswordHashString) -> Result<()> {
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), &password_hash_string.password_hash())
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher, PasswordVerifier};
    use uuid::Uuid;

    #[test]
    pub fn argon2_test() {
        println!("{}", Uuid::now_v7().to_string());
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
