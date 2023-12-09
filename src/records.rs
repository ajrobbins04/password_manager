
pub mod records {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccountInfo {
        pub account: String,
        pub username: String,
        pub password: String
    }
}
