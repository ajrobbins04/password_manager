
pub mod records {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccountInfo {
        pub account: String,
        pub username: String,
        pub password: String
    }

    pub fn generate_password() -> Vec<char> {
        // Define a vector containing characters for password generation
        let chars_pool: Vec<char> = (b'A'..=b'Z')
            // ..= makes the last range value inclusive
            .chain(b'a'..=b'z') 
            .chain(b'0'..=b'9')
            .chain(b'!'..=b'/')
            .chain(b':'..=b'@')
            .chain(b'['..=b'`')
            .chain(b'{'..=b'~')
            .map(|c| c as char)
            .collect();

        chars_pool
    } 
}
