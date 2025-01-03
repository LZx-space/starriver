use std::fmt::Display;
use std::str::FromStr;

use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;
use sea_orm::DbErr;
use serde::Serialize;
use uuid::Uuid;

use crate::infrastructure::model::page::{PageQuery, PageResult};
use crate::infrastructure::service::dictionary::dictionary_repository::Repository;

pub struct Dictionary {
    repo: Repository,
}

impl Dictionary {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        let repo = Repository::new(conn);
        Dictionary { repo }
    }

    pub async fn page(&self, query: PageQuery) -> Result<PageResult<DictionaryEntry>, DbErr> {
        self.repo.paging(query).await
    }

    pub async fn insert(&self, e: DictionaryEntry) -> Option<DbErr> {
        self.repo.insert(e).await
    }

    pub async fn update(&self, e: DictionaryEntry) -> Option<DbErr> {
        todo!()
    }

    pub async fn delete(&self, e: DictionaryEntry) -> Option<DbErr> {
        todo!()
    }
}

#[derive(Serialize)]
pub struct DictionaryEntry {
    pub id: Uuid,
    pub value: String,
    pub data_type: DataType,
    pub comment: String,
    pub create_at: DateTime<Local>,
    pub update_at: Option<DateTime<Local>>,
}

impl DictionaryEntry {
    pub fn new(value: String, data_type: DataType, comment: String) -> Result<Self, String> {
        match match data_type {
            DataType::I8 => value.parse::<i8>().map_err(|e| e.to_string()).err(),
            DataType::I16 => value.parse::<i16>().map_err(|e| e.to_string()).err(),
            DataType::I32 => value.parse::<i32>().map_err(|e| e.to_string()).err(),
            DataType::I64 => value.parse::<i64>().map_err(|e| e.to_string()).err(),
            DataType::I128 => value.parse::<i128>().map_err(|e| e.to_string()).err(),
            DataType::ISIZE => value.parse::<isize>().map_err(|e| e.to_string()).err(),
            DataType::U8 => value.parse::<u8>().map_err(|e| e.to_string()).err(),
            DataType::U16 => value.parse::<u16>().map_err(|e| e.to_string()).err(),
            DataType::U32 => value.parse::<u32>().map_err(|e| e.to_string()).err(),
            DataType::U64 => value.parse::<u64>().map_err(|e| e.to_string()).err(),
            DataType::U128 => value.parse::<u128>().map_err(|e| e.to_string()).err(),
            DataType::USIZE => value.parse::<usize>().map_err(|e| e.to_string()).err(),
            DataType::F32 => value.parse::<f32>().map_err(|e| e.to_string()).err(),
            DataType::F64 => value.parse::<f64>().map_err(|e| e.to_string()).err(),
            DataType::BOOLEAN => value.parse::<bool>().map_err(|e| e.to_string()).err(),
            DataType::STRING => None,
        } {
            None => {
                let id = Uuid::now_v7();
                Ok(DictionaryEntry {
                    id,
                    value,
                    data_type,
                    comment,
                    create_at: Local::now(),
                    update_at: None,
                })
            }
            Some(err) => Err(err),
        }
    }

    pub fn try_parse<T: FromStr>(self) -> Result<T, <T as FromStr>::Err> {
        self.value.parse::<T>()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
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

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DataType::I8 => "I8".to_string(),
            DataType::I16 => "I16".to_string(),
            DataType::I32 => "I32".to_string(),
            _ => "".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for DataType {
    type Err = DbErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I8" => Ok(DataType::I8),
            _ => Err(DbErr::Custom("错误的数据类型".to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;

    use crate::infrastructure::service::dictionary::dictionary_service::{
        DataType, DictionaryEntry,
    };

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
        println!("obj->{:?}", obj);

        match "u123".parse::<i8>() {
            Ok(i) => {
                println!("parse to i8 {}", i);
            }
            Err(err) => {
                println!("{}", err);
            }
        }

        let result = DictionaryEntry::new("55".to_string(), DataType::I8, "测试".to_string());
        assert!(result.is_ok());
        let result = DictionaryEntry::new("a55".to_string(), DataType::I8, "测试".to_string());
        assert!(result.is_err());
        let result = DictionaryEntry::new("66".to_string(), DataType::BOOLEAN, "测试".to_string());
        assert!(result.is_err());
        let result = DictionaryEntry::new("127".to_string(), DataType::I8, "测试".to_string());
        println!("parse-{:?}", result.unwrap().try_parse::<isize>());
    }
}
