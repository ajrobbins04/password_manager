/* records module deals with structs and methods
where information must be recorded in the manager 
database or retrieved from it. */

pub mod records {
    
    use rusqlite::{Connection, Result, Statement, Error, Rows};
    use rusqlite::types::{FromSql, ValueRef, FromSqlError, FromSqlResult};

    use std::fs::File;
    use std::cell::Cell; // bends rules of immutability to change userId value

    #[derive(Debug)] // gives the derived trait to AccountInfo
    pub struct AccountInfo {
        pub account: String,
        pub username: String,
        pub password: String,
        pub accountId: Option<u64> // only applies to accounts pulled from db
    }

    pub struct User {
        // value is None if a user is not logged in or an unsigned int up to 255
        pub client_id: Option<u8>
    }

    // a default trait helps avoid ownership-related 
    // issues by declaring variables prior to being 
    // assigned values
    pub trait Default {
        fn default() -> Self;
    }

    impl Default for AccountInfo {
        fn default() -> Self {
            Self {
                account: String::from("default_account"),
                username: String::from("default_username"),
                password: String::from("default_password"),
                accountId: None
            }
        }
    }
    impl Default for User {
        fn default() -> Self {
            Self {
                client_id: None
            }
        }
    }

    // contains methods necessary for retrieving data from db
    pub trait Retrieve {
        fn lookup_user(&self, username_input: &str, password_input: &str) -> Result<(Option<u8>,)>;
        fn set_client_id(&mut self, id: Option<u8>);
        fn get_id(&self) -> u8;
        fn get_accounts(&self, conn: &Connection, client_id: &str) -> Result<Vec<AccountInfo>>;
    }

    impl Retrieve for User {

        fn lookup_user(&self, username_input: &str, password_input: &str) -> Result<(Option<u8>,)> {
            let conn = Connection::open("manager.db")?;
        
            let sql = "SELECT clientId FROM clients WHERE username = ? AND password = ?";
        
            // final argument ensures that only one row at the most is found (as it should be anyways)
            let client_id: u8 = conn.query_row(sql, [username_input, password_input],|row| row.get(0))?;
          
            // will only be called if a user_id is found due to '?' error propagation
            Ok((Some(client_id),))
        }

        fn set_client_id(&mut self, id: Option<u8>) {
            self.client_id = id;
        }

        // returns client id for current user, unless an id cannot be found
        fn get_id(&self) -> u8 {
            match self.client_id {
                Some(value) => value,
                None => 0
            }
        }
        fn get_accounts(&self, conn: &Connection, client_id: &str) -> Result<Vec<AccountInfo>> {
    
            let mut stmt = conn.prepare("SELECT accountName, accountUsername, accountPassword, accountId
            FROM accounts WHERE clientId = ?")?;

            let accounts: Result<Vec<AccountInfo>, rusqlite::Error> = stmt.query_map([client_id], |row| {

                // use the iterator returned by query_map to create an instance of AccountInfo
                Ok(AccountInfo {
                    account: row.get(0)?,
                    username: row.get(1)?,
                    password: row.get(2)?,
                    accountId: row.get(3)?
                })
            })?.collect(); // add each new instance of AccountInfo to the accounts vector
            
            accounts
        }
    }

    
    // contains methods necessary for transferring data to db
    pub trait Transfer {
       fn add_account(entry: AccountInfo, id: &Option<u8>) -> Result<()>;
       fn edit_account(entry: AccountInfo, id: &Option<u8>) -> Result<()>;
    }

    impl Transfer for AccountInfo {
        fn add_account(entry: AccountInfo, id: &Option<u8>) -> Result<()> {
            let conn = Connection::open("manager.db")?; 
            let stmt = "INSERT INTO accounts (accountName, accountUsername, accountPassword, clientId)
            VALUES (?, ?, ?, ?)";
            let client_id_string = id.unwrap().to_string(); // unwrap to retrieve the 8-bit value of Some
            conn.execute(stmt, [entry.account, entry.username, entry.password, client_id_string])?;
            Ok(())
        }

        fn edit_account(entry: AccountInfo, id: &Option<u8>) -> Result<()> {
            let conn = Connection::open("manager.db")?; 
            let stmt = "INSERT INTO accounts (accountName, accountUsername, accountPassword, clientId)
            VALUES (?, ?, ?, ?)";
            let client_id_string = id.unwrap().to_string();
            conn.execute(stmt, [entry.account, entry.username, entry.password, client_id_string])?;
            Ok(())
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

    pub fn open_database() -> Result<Connection, Error> {
       
        let conn: Connection = Connection::open("manager.db")?;
        Ok(conn)
    }
    
}
