/* records module deals with structs and methods
where information must be recorded in the manager 
database or retrieved from it. */

pub mod records {
    use rand::Rng; // for random password generation
    use rusqlite::{Connection, Result, Error};

    
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

    pub struct PasswordSpecs {
        lower_letters: u8,
        upper_letters: u8,
        numbers: u8,
        special_chars: u8
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
    impl Default for PasswordSpecs {
        fn default() -> Self {
            Self {
                lower_letters: 8,
                upper_letters: 2,
                numbers: 4,
                special_chars: 4
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
       fn add_account(conn: &Connection, entry: AccountInfo, id: &u8) -> Result<()>;
    }

    impl Transfer for AccountInfo {
        fn add_account(conn: &Connection, entry: AccountInfo, id: &u8) -> Result<()> {
            let stmt = "INSERT INTO accounts (accountName, accountUsername, accountPassword, clientId)
            VALUES (?, ?, ?, ?)";
            let client_id_string = id.to_string(); 
           
            conn.execute(stmt, [entry.account, entry.username, entry.password, client_id_string])?;

            Ok(()) // only an indication of success needs to be sent back
        }
    }
 
    pub fn generate_password(length: u8) -> String {

        // Define a vector containing characters for password generation
        let chars_pool: Vec<char> = (b'A'..=b'Z')
        
            // adding b means everything is accessed by its ascii values
            // the ranges shown are found using a chart of ascii values
            .chain(b'a'..=b'z') 
            .chain(b'0'..=b'9')
            .chain(b'!'..=b'/')
            .chain(b':'..=b'@')
            .chain(b'['..=b'`')
            .chain(b'{'..=b'~')
            // convert every byte into a char
            .map(|c| c as char)
            .collect(); // place all char values in the chars_pool vector
        
        // initializes a random number generator
        let mut rng = rand::thread_rng();

        // generates password w/specified length by repeatedly selecting random characters
        // from the chars_pool vector and collecting them into the password String
        let password: String = (0..length).map(|_| chars_pool[rng.gen_range(0..chars_pool.len())]).collect();
        password
    } 
    
    pub fn open_database() -> Result<Connection, Error> {
       
        let conn: Connection = Connection::open("manager.db")?;
        Ok(conn)
    }

    }

        