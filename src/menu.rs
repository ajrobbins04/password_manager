/* menu module primarily deals with interacting with the  
   user by displaying output and receiving input.     */

// Rust's module paths do not correspond w/the project's file paths
pub mod menu {
    use std::io; // input/output functionality
    use std::io::Write;
    use crate::records::records::{AccountInfo, User, Transfer, open_database, Default, Retrieve, generate_password}; // 'crate' begins module search at root of project

    pub fn run_main_menu() {
        let mut run_program = true;
        
        loop {
            // '!' denotes that println! is a macro.
            print!("Enter 'y' to login or 'n' to close the program: ");
            let input = get_one_letter_input();

            match input.as_str() {
                "y" => { 
                    println!(); 
                    run_login_menu();
                },
                "n" => {
                    run_program = false;
                }
                _ => {
                    println!();
                    println!("ERROR: Invalid input detected. Please enter 'y' to login or 'n' to logout.");
                }
            } 
            if !run_program { // end program
                println!();
                println!("Goodbye!");
                break; 
            }
        }
    }
    pub fn run_login_menu() {
        let mut run_menu = true; 
        let mut username_input = String::new();
        let mut password_input = String::new();
        let mut user = User::default();
        loop {   
            println!("Password Manager Login");
            println!();
            print!("Enter Username: ");
            username_input = get_input();
            
            if username_input.is_empty() {
                println!();
                println!("ERROR: No username entered. Please try again.");
            }
            else {
                println!();
                print!("Enter Password: ");
                password_input = get_input();

                if password_input.is_empty() {
                    println!();
                    println!("ERROR: No password entered. Please try again.");
                }
                else {
                    match user.lookup_user(&username_input, &password_input) {
                        // use wildcard, since value has already been set as user_id
                        // in lookup_user & no longer matters
                        Ok((Some(client_id),))=> {
                     
                            println!();
                            println!("You are logged in!");
                    
                            user.set_client_id(Some(client_id));
                           
                            // only stops running when user wants to logout
                            run_logged_in_menu(&user);
                            user.set_client_id(None); // Option can either be None or Some()
                            run_menu = false;
                            println!();
                            println!("Logout successful");

                        }
                        Ok((None,)) => {
                            println!();
                            println!("The login attempt failed. Please try again.");
                            run_main_menu();
                        }
                        Err(e) => {
                            println!();
                            println!("An error occurred during the login attempt. Please try again.");
                            run_main_menu();
                        }
                    }
                    if !run_menu {
                        break;
                    }
                }
            }
        }
    }
    pub fn run_logged_in_menu(user: &User) {
        // exit condition for the loop
        let mut run_menu: bool = true;

        loop {  
            println!(); 
            println!("Password Manager Menu:");  
            println!("1. Add New Entry");
            println!("2. View All Entries");
            println!("3. Edit an Entry");
            println!("4. Delete an Entry");
            println!("5. Logout");
            println!();
            print!("Enter your selection from 1-5: ");

            // will need to allocate data from the heap for a String
            let mut input = get_input();

            match input.as_str() {
                "1" => {
                    add_entry_menu(&user);
                },
                "2" => {
                    let conn = open_database();

                    match conn {
                        Ok(conn) => {
                            // used to retrieve just the accounts that belong to the user
                          
                            let client_id = user.get_id().to_string();
                            match user.get_accounts(&conn, &client_id) {
                                
                                Ok(accounts) => {
                                    let mut num: u8 = 1;
                                    for account in accounts.iter() {
                                        println!();
                                        println!("Account {}: {}", num, account.account);
                                        println!("Username: {}", account.username);
                                        println!("Password: {}", account.password);
                                        num += 1;
                                    }
                                }
                                Err(e) => {
                                    println!();
                                    println!("An error occurred when retrieving accounts from the database.");
                                   
                                }
                            }
                        }
                        Err(e) => {
                            println!();
                            println!("An error occurred when connecting to the database.");
                          
                        }
                    }
                },
                "3" => {
                    println!("3!")
                },
                "4" => {
                    println!("4!")
                },
                "5" => {  // logout 
                    run_menu = false; 
                }
                _ => { // wildcard input
                    println!("ERROR: Invalid input detected. Please enter a number from 1 - 5.");
                    println!();
                }
            }
            if !run_menu {
                break;
            }
        }
    }

    pub fn add_entry_menu(curr_user: &User) {
        let mut run_options: bool = true;

        loop {
            println!();
            println!("Would you like the system to generate a password for you?");
            print!("Enter (y/n), or enter q to return to the main menu: ");

            let input = get_one_letter_input();

            match input.as_str() {
                "y" => { 
                    let entry: AccountInfo = prompt_account_info_some();
                    if let Ok(_) = AccountInfo::add_account(entry, &Some(curr_user.get_id())){
                        println!();
                        println!("New account successfully added!");
                        println!();
                        print!("Would you like to add another account? Enter (y/n): ");
                        let input = get_one_letter_input();
                        if input == "y" {
                            add_entry_menu(&curr_user);
                        }
                        else {
                            run_options = false;
                        }
                    } else {
                        run_options = false;
                    }
                },
                "n" => {
                    let entry: AccountInfo = prompt_account_info_all();
                    if let Ok(_) = AccountInfo::add_account(entry, &Some(curr_user.get_id())){
                        println!();
                        println!("New account successfully added!");
                        println!();
                        print!("Would you like to add another account? Enter (y/n): ");
                        let input = get_one_letter_input();
                        if input == "y" {
                            add_entry_menu(&curr_user);
                        }
                        else {
                            run_options = false;
                        }
                    } else {
                        run_options = false;
                    }
                }
                "q" => {
                    run_options = false;
                }
                _ => {
                    println!("ERROR: Invalid input detected. Please enter 'y', 'n', or 'q' to return to the main menu.");
                }
            }

            if !run_options {
                break;
            }
        }
    }
    // retrieves user input to create a new account entry
    pub fn prompt_account_info_all() -> AccountInfo {
        let mut entry = AccountInfo::default();
        let mut complete: bool = false;

        loop {
            
            let account_name = get_account_name();
            let username = get_username();
            let password = get_password();

            if !account_name.is_empty() && !username.is_empty() && !password.is_empty() {
                
                entry = AccountInfo {      // entry takes ownership of all 4 values
                    account: account_name,
                    username,
                    password,
                    accountId: None  // id gets assigned after entry is added to db
                };
                complete = true;
            }
            if complete {
                break;
            }
        }
        entry

    }

    pub fn prompt_account_info_some() -> AccountInfo {
        let mut entry = AccountInfo::default();
        let mut complete: bool = false;

        loop {
            
            let account_name = get_account_name();
            let username = get_username();
            let password = get_password_generate();

            if !account_name.is_empty() && !username.is_empty() {

                
                entry = AccountInfo {      // entry takes ownership of all 4 values
                    account: account_name,
                    username,
                    password,
                    accountId: None  // id gets assigned after entry is added to db
                };
                complete = true;
            }
            if complete {
                break;
            }
        }
        entry

    }
    fn get_input() -> String {
        // always flush the buffer before receiving new input
        io::stdout().flush().expect("Failed to flush stdout");

        // mut makes the variable mutable so the input can change
        let mut input = String::new();
        // passes in input as a mutable reference
        io::stdin().read_line(&mut input).expect("Failed to read line"); // .expect used for input stream error handling
        // trim converts input to &str when removing output, 
        // so to_string() must convert it back to a String
        input.trim().to_string()
    }

    // input requires some manipulation to return single letters (i.e. 'y' or 'n')
    fn get_one_letter_input() -> String {

        io::stdout().flush().expect("Failed to flush stdout");
        // same as get_input
        let mut input = String::new();

        // must remove whitespace before retrieving first character from input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string();
        input.to_lowercase().chars().nth(0).unwrap_or('\0').to_string() // no semicolon to return value
    }

    fn get_account_name() -> String {

        io::stdout().flush().expect("Failed to flush stdout");
        let mut account_name_empty: bool = true;

        // loses ownership too soon if defined inside loop
        let mut account_name = String::new();

        loop {
            println!(); 
            print!("Enter the name of your account: ");
            account_name = get_input();
            if account_name.is_empty() {
                println!("ERROR: No account name entered. Please try again.");
                println!();
            } else {
                account_name_empty = false;
            }

            if !account_name_empty {
                break;
            }
        }
        account_name // return
    }

    fn get_username() -> String {

        io::stdout().flush().expect("Failed to flush stdout");
        let mut username_empty: bool = true;

        // loses ownership too soon if defined inside loop
        let mut username = String::new();

        loop {
            print!("Enter your account username: ");
            username = get_input();
            if username.is_empty() {
                println!("ERROR: No username entered. Please try again.");
                println!();
            } else {
                username_empty = false;
            }

            if !username_empty {
                break;
            }
        }
        username
    }

    fn get_password() -> String {

        io::stdout().flush().expect("Failed to flush stdout");
        let mut password_empty: bool = true;
        let mut password = String::new();

        loop {
            println!(); 
            print!("Enter your account password: ");
            password = get_input();
            if password.is_empty() {
                println!("ERROR: No password entered. Please try again.");
                println!();
            } else {
                password_empty = false;
            }

            if !password_empty {
                break;
            }
        }

        password
    }

    fn get_password_generate() -> String {

        io::stdout().flush().expect("Failed to flush stdout");
        let mut password_empty: bool = true;
        let mut password = String::new();

        loop {
            println!(); 
            print!("Enter the length of your password: ");
            let length_string = get_input();
            match length_string.parse::<u8>() {
                Ok(length) => {
                    password = generate_password(length);
                }
                Err(e) => {
                    println!("ERROR: The new password length could not be found.");
                }
            }
            
            if password.is_empty() {
                println!("ERROR: Password generation failed.");
                println!();
            } else {
                password_empty = false;
            }

            if !password_empty {
                break;
            }
        }

        password
    }

}