use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use std::time::SystemTime;

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
