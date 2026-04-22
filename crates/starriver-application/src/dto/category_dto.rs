pub mod req {
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    pub struct CreateOrUpdateCategoryCmd {
        #[validate(length(min = 1, max = 10))]
        pub name: String,
    }
}

pub mod res {}
