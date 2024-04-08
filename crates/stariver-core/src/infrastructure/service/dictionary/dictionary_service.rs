use std::any::Any;
use std::str::FromStr;

use serde::Serialize;
use crate::infrastructure::model::page::{PageQuery, PageResult};

pub struct Dictionary {

}

impl Dictionary {

    pub fn page(query: PageQuery) -> PageResult<DictionaryEntry> {
        todo!()
    }

}

#[derive(Serialize, Debug)]
pub struct DictionaryEntry {
    id: String,
    value: String,
    data_type: DataType,
    comment: String,
}

impl DictionaryEntry {
    pub fn new(id: String, value: String, data_type: DataType, comment: String) -> Result<Self, String> {
        match match data_type {
            DataType::I8 => {
                value.parse::<i8>().map_err(|e| e.to_string()).err()
            }
            DataType::I16 => {
                value.parse::<i16>().map_err(|e| e.to_string()).err()
            }
            DataType::I32 => {
                value.parse::<i32>().map_err(|e| e.to_string()).err()
            }
            DataType::I64 => {
                value.parse::<i64>().map_err(|e| e.to_string()).err()
            }
            DataType::I128 => {
                value.parse::<i128>().map_err(|e| e.to_string()).err()
            }
            DataType::ISIZE => {
                value.parse::<isize>().map_err(|e| e.to_string()).err()
            }
            DataType::U8 => {
                value.parse::<u8>().map_err(|e| e.to_string()).err()
            }
            DataType::U16 => {
                value.parse::<u16>().map_err(|e| e.to_string()).err()
            }
            DataType::U32 => {
                value.parse::<u32>().map_err(|e| e.to_string()).err()
            }
            DataType::U64 => {
                value.parse::<u64>().map_err(|e| e.to_string()).err()
            }
            DataType::U128 => {
                value.parse::<u128>().map_err(|e| e.to_string()).err()
            }
            DataType::USIZE => {
                value.parse::<usize>().map_err(|e| e.to_string()).err()
            }
            DataType::F32 => {
                value.parse::<f32>().map_err(|e| e.to_string()).err()
            }
            DataType::F64 => {
                value.parse::<f64>().map_err(|e| e.to_string()).err()
            }
            DataType::BOOLEAN => {
                value.parse::<bool>().map_err(|e| e.to_string()).err()
            }
            DataType::STRING => {
                None
            }
        } {
            None => { Ok(DictionaryEntry { id, value, data_type, comment }) }
            Some(err) => { Err(err) }
        }
    }

    pub fn value<T: FromStr>(self) -> Result<T, <T as FromStr>::Err> {
        self.value.parse::<T>()
    }
}

#[derive(Serialize, Debug)]
pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISIZE,
    U8,
    U16,
    U32,
    U64,
    U128,
    USIZE,
    F32,
    F64,
    BOOLEAN,
    STRING,
}

#[test]
fn test_de() {
    #[derive(Serialize, Debug)]
    struct A {
        a: String,
        b: u32,
    }
    let obj = A {
        a: "abc".to_string(),
        b: 100,
    };
    let result = serde_json::to_string_pretty(&obj)
        .expect("");
    println!("obj->{:?}", obj);

    println!("c-{:?}", 'a'.type_id());

    println!("s-{:?}", result.to_string().type_id());

    println!("b-{:?}", true.type_id());

    println!("i-{:?}", 1i8.type_id());
    println!("i-{:?}", 1i16.type_id());
    println!("i-{:?}", 1i32.type_id());
    println!("i-{:?}", 1i64.type_id());
    println!("i-{:?}", 1i128.type_id());
    println!("i-{:?}", 1isize.type_id());

    println!("f-{:?}", 1f32.type_id());
    println!("f-{:?}", 1f64.type_id());

    println!("t-{:?}", (500, 6.4, 1).type_id());
    println!("a-{:?}", [1, 2, 3, 4, 5].type_id());

    match "u123".parse::<i8>() {
        Ok(i) => { println!("parse to i8 {}", i); }
        Err(err) => { println!("{}", err); }
    }

    let result = DictionaryEntry::new("测试1".to_string(), "55".to_string(), DataType::I8, "测试".to_string());
    println!("1-{:?}", result);
    let result = DictionaryEntry::new("测试2".to_string(), "a55".to_string(), DataType::I8, "测试".to_string());
    println!("2-{:?}", result);
    let result = DictionaryEntry::new("测试3".to_string(), "66".to_string(), DataType::BOOLEAN, "测试".to_string());
    println!("3-{:?}", result);
    let result = DictionaryEntry::new("测试3".to_string(), "66".to_string(), DataType::I8, "测试".to_string());
    println!("4-{:?}", result);
}