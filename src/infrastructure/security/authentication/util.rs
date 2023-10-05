use argon2::password_hash::SaltString;
use argon2::{
    password_hash::errors::Result, Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use std::time::SystemTime;

pub struct PasswordUtils<'a> {
    argon2: Argon2<'a>,
}

impl<'a> Default for PasswordUtils<'a> {
    fn default() -> Self {
        PasswordUtils {
            argon2: Default::default(),
        }
    }
}

impl PasswordUtils {
    pub fn hash(&self, salt: &str, password: &str) -> Result<PasswordHash> {
        let salt_string = SaltString::encode_b64(salt.as_bytes());
        self.argon2.hash_password(password.as_bytes(), &salt_string)
    }

    pub fn verify(&self, password: &str, hash: &PasswordHash<'_>) -> Result<()> {
        self.argon2.verify_password(password.as_bytes(), &hash)
    }
}

#[test]
pub fn argon2_test() {
    let start = SystemTime::now();
    let argon2 = Argon2::default();
    let pwd_bytes = "abc@op/9012345".as_bytes();
    let salt_string = SaltString::encode_b64("ABCDEFGH".as_bytes()).expect("啊啊啊");
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
