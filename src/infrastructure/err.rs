#[derive(Debug)]
pub struct BizErr {
    pub msg: String,
}

impl BizErr {
    pub fn new(msg: String) -> Self {
        BizErr { msg }
    }
}
