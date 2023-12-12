/* records module deals with structs and methods
where information must be recorded in the manager 
database or retrieved from it. */

pub mod records {
    
    use rusqlite::{Connection, Result};
    use rusqlite::Error;
    use std::fs::File;
    use std::cell::Cell; // bends rules of immutability to change userId value

    #[derive(Debug)] // gives the derived trait to AccountInfo
    pub struct AccountInfo {
        pub account: String,
        pub username: String,
        pub password: String
    }

    thread_local! {
        // value is None if a user is not logged in or an unsigned int up to 255
        static USER_ID: Cell<Option<u8>> = Cell::new(None);
    }

    fn set_user_id(id: Option<u8>) {
        USER_ID.with(|cell| { // closures (denoted using ||) are similar to anonymous functions
            cell.set(id); // variants of Option are either Some or None
        })
    }
  
    pub fn lookup_user(username_input: &str, password_input: &str) -> Result<()> {
        let conn = Connection::open("manager.db")?;
    
        let sql = "SELECT clientId FROM clients WHERE username = ? AND password = ?";
    
        // final argument ensures that only one row at the most is found (as it should be anyways)
        let user_id: u8 = conn.query_row(sql, [username_input, password_input], |row| row.get(0))?;
        
        // will only be called if a user_id is found due to '?' error propagation
        set_user_id(Some(user_id)); // must convert user_id to Option<u8>
        Ok(())
    }

    // retrieves current value for USER_ID
    fn get_user_id() -> Option<u8> {
        USER_ID.with(|cell| {
            cell.get()
        })
    }

    trait Transfer {
       fn add_account() -> Result<()>;
    }

    /*impl Transfer for AccountInfo {
        fn add_account(entry: AccountInfo) -> Result<()> {
            let conn = Connection::open("manager.db")?; 
            let stmt: &str = "INSERT INTO accounts ()"
        }
    }*/

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
    
    fn read_sql_from_file(path: &str) -> String {
        let mut file: File = File::open(path).unwrap();
        let mut contents: String = String::new();
        contents
    }
}
