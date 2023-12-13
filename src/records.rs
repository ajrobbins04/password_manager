/* records module deals with structs and methods
where information must be recorded in the manager 
database or retrieved from it. */

pub mod records {
    
    use rusqlite::{Connection, Result, Statement, Error, Rows};
    use std::fs::File;
    use std::cell::Cell; // bends rules of immutability to change userId value

    #[derive(Debug)] // gives the derived trait to AccountInfo
    pub struct AccountInfo {
        pub client_id: u8,
        pub account: String,
        pub username: String,
        pub password: String
    }

    pub struct User {
        pub client_id: u8
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
                client_id: 0,
                account: String::from("default_account"),
                username: String::from("default_username"),
                password: String::from("default_password"),
            }
        }
    }
    impl Default for User {
        fn default() -> Self {
            Self {
                client_id: 0
            }
        }
    }

    thread_local! {
        // value is None if a user is not logged in or an unsigned int up to 255
        static USER_ID: Cell<Option<u8>> = Cell::new(None);
    }

    // contains methods necessary for retrieving data from db
    pub trait Retrieve {
        fn set_client_id(&self, id: &Option<u8>);
        fn get_accounts(&self) -> Result<Rows, Error>;

    }

    impl Retrieve for User {
        fn set_client_id(&self, id: &Option<u8>) {
            USER_ID.with(|cell| { // closures (denoted using ||) are similar to anonymous functions
                cell.set(*id); // variants of Option are either Some or None
            })
        }
        fn get_accounts(&self) -> Result<Rows, Error> {
            let conn = Connection::open("manager.db")?; 
            let stmt = "SELECT accountName, accountUsername, accountPassword
            FROM accounts WHERE clientId = ?";

            // will be used to retrieve just the accounts that belong to the user
            let client_id_string = self.client_id.to_string();

            // must prepare statement when query returns rows
            let mut sql_query: Statement = conn.prepare(stmt)?;
            let rows = sql_query.query([client_id_string])?;
     
            Ok((rows))
        }
    }

    pub fn set_client_id(id: &Option<u8>) {
        USER_ID.with(|cell| { // closures (denoted using ||) are similar to anonymous functions
            cell.set(*id); // variants of Option are either Some or None
        })
    }
  
    pub fn lookup_user(username_input: &str, password_input: &str) -> Result<(Option<u8>,)> {
        let conn = Connection::open("manager.db")?;
    
        let sql = "SELECT clientId FROM clients WHERE username = ? AND password = ?";
    
        // final argument ensures that only one row at the most is found (as it should be anyways)
        let client_id: u8 = conn.query_row(sql, [username_input, password_input], |row| row.get(0))?;
      
        // will only be called if a user_id is found due to '?' error propagation
        set_client_id(&Some(client_id)); // must convert user_id to Option<u8>
    
        Ok((Some(client_id),))
    }

    // retrieves current value for USER_ID
    pub fn get_client_id() -> Option<u8> {
        USER_ID.with(|cell| {
            cell.get()
        })
    }
    // contains methods necessary for transferring data to db
    pub trait Transfer {
       fn add_account(entry: AccountInfo) -> Result<()>;
       fn edit_account(entry: AccountInfo) -> Result<()>;
       fn set_client_id(id: Option<u8>);
    }

    impl Transfer for AccountInfo {
        fn add_account(entry: AccountInfo) -> Result<()> {
            let conn = Connection::open("manager.db")?; 
            let stmt = "INSERT INTO accounts (accountName, accountUsername, accountPassword, clientId)
            VALUES (?, ?, ?, ?)";
            let client_id_string = entry.client_id.to_string();
            conn.execute(stmt, [entry.account, entry.username, entry.password, client_id_string])?;
            Ok(())
        }

        fn edit_account(entry: AccountInfo) -> Result<()> {
            let conn = Connection::open("manager.db")?; 
            let stmt = "INSERT INTO accounts (accountName, accountUsername, accountPassword, clientId)
            VALUES (?, ?, ?, ?)";
            let client_id_string = entry.client_id.to_string();
            conn.execute(stmt, [entry.account, entry.username, entry.password, client_id_string])?;
            Ok(())
        }
        fn set_client_id(id: Option<u8>) {
            USER_ID.with(|cell| { // closures (denoted using ||) are similar to anonymous functions
                cell.set(id); // variants of Option are either Some or None
            })
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
