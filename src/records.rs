
pub mod records {

    use rusqlite::{Connection, Result};
    use std::fs::File;

    #[derive(Debug)] // gives the derived trait to AccountInfo
    pub struct AccountInfo {
        pub account: String,
        pub username: String,
        pub password: String
    }

    trait Transfer {
        fn convert_to_json(&self) -> String;
    }

    // implements a Default trait for AccountInfo
    // to avoid ownership-related issues by waiting
    // to create a struct until has its values
    impl Default for AccountInfo {
        fn default() -> Self {
            Self {
                account: String::from("default_account"),
                username: String::from("default_username"),
                password: String::from("default_password"),
            }
        }
    }

    pub fn generate_password() -> Vec<char> {
        // Define a vector containing characters for password generation
        let chars_pool: Vec<char> = (b'A'..=b'Z')
            // ..= makes the final value in every range inclusive
            .chain(b'a'..=b'z') 
            .chain(b'0'..=b'9')
            .chain(b'!'..=b'/')
            .chain(b':'..=b'@')
            .chain(b'['..=b'`')
            .chain(b'{'..=b'~')
            // convert every byte into a char
            .map(|c| c as char)
            .collect(); // place all char values in the chars_pool vector
        
        for c in &chars_pool {
            print!("{}", c);
        }
        chars_pool
    } 

    
   /*  pub fn get_all_account_info() -> Vec<AccountInfo> {
        let mut accounts: Vec<AccountInfo> = Vec::new();
        let mut not_complete: bool = true;

        loop {
            let account_name = get_account_name();
            let username = get_username();
            let password = get_password();

            if !account_name.is_empty() && !username.is_empty()
            && !password.is_empty() {
                let entry = AccountInfo {
                    account: account_name,
                    username,
                    password,
                };
                accounts.push(entry);

                println!("Account added successfully!");
                println!();
            }
        }accounts

    
    }*/
    
    fn read_sql_from_file(path: &str) -> String {
        let mut file: File = File::open(path).unwrap();
        let mut contents: String = String::new();
        contents
    }
}
