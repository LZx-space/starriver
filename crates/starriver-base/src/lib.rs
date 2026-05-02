// 允许基础设施内部使用 axum 原生提取器
#![allow(clippy::disallowed_types)]

pub mod db;
pub mod dto;
pub mod error;
pub mod error_mapping;
pub mod extract;
pub mod model;
pub mod query;
pub mod repository;
pub mod security;
pub mod service;
pub mod util;
pub mod visualization;
