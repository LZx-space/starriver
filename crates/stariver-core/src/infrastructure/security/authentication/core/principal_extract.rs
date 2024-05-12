use std::future::Future;

use crate::infrastructure::security::authentication::core::principal::Principal;

pub trait Extractor: Principal + Sized {
    type Payload;

    type Error;

    type Future: Future<Output = Result<Self, Self::Error>>;

    fn from_payload(payload: &Self::Payload) -> Self::Future;
}
