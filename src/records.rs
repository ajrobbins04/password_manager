
pub mod account {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct accountInfo {
        pub account: String,
        pub username: String,
        pub password: String
    }
}
