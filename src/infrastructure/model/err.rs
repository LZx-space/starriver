use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct BizErr {
    pub msg: String,
}

impl BizErr {
    pub fn new(msg: String) -> Self {
        BizErr { msg }
    }
}

impl Display for BizErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for BizErr {}
