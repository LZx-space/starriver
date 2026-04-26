use sea_orm::{FromQueryResult, TryGetable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct IdName<T: TryGetable> {
    pub id: T,
    pub name: String,
}
