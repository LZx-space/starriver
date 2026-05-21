use std::fmt::Debug;

use serde::{Deserialize, Serialize};

/// 主体：能够进行身份验证的用户或应用程序
pub trait Principal: Send + Sync {
    type Id;

    type Authority: Authority;

    fn id(&self) -> &Self::Id;

    fn authorities(&self) -> Vec<&Self::Authority>;
}

pub trait Authority: Send {
    fn name(&self) -> &String;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimpleAuthority {
    name: String,
}

impl Authority for SimpleAuthority {
    fn name(&self) -> &String {
        &self.name
    }
}
