use std::any::Any;
use std::fmt::Error;

use serde::Serialize;

pub struct Dictionary {
    // entry_vec: Vec<DictionaryEntry>,
}

impl Dictionary {}

pub struct DictionaryEntry<T> {
    id: String,
    value: T,
    comment: String,
}

impl<T: Serialize> DictionaryEntry<T> {
    pub fn new(id: String, value: T, comment: String) -> Result<Self, Error> {
        Ok(DictionaryEntry { id, value, comment })
    }
}

#[test]
fn test_de() {
    #[derive(Serialize)]
    struct A {
        a: String,
        b: u32,
    }
    let result = serde_json::to_string_pretty(&A {
        a: "abc".to_string(),
        b: 100,
    })
    .expect("");
    println!("str->{}", result);
    println!("0-{:?}", result.type_id());
    println!("1-{:?}", 1.type_id());
    println!("2-{:?}", 'a'.type_id());
    println!("3-{:?}", "abc".to_string().type_id());
    println!("4-{:?}", true.type_id());
    println!("5-{:?}", 2f32.type_id());
}
