use serde::Serialize;
use std::fmt::Debug;

/// 凭证
pub trait Credentials: Serialize + Debug {}
