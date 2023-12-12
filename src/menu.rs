/* menu module primarily deals with interacting with the  
   user by displaying output and receiving input.     */

// Rust's module paths do not correspond w/the project's file paths
pub mod menu {
    use std::io; // input/output functionality
    use std::io::Write;
    use crate::records::records::{AccountInfo, Transfer, lookup_user, set_client_id, get_client_id}; // 'crate' begins module search at root of project

    pub fn run_main_menu() {
        let mut run_program = true;
        
        loop {
            // '!' denotes that println! is a macro.
            println!("Enter 'y' to login or 'n' to close the program: ");
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
        loop {   
            println!("Password Manager Login");
            println!();
            println!("Enter Username: ");
            username_input = get_input();
            
            if username_input.is_empty() {
                println!();
                println!("ERROR: No username entered. Please try again.");
            }
            else {
                println!();
                println!("Enter Password: ");
                password_input = get_input();

                if password_input.is_empty() {
                    println!();
                    println!("ERROR: No password entered. Please try again.");
                }
                else {
                    match lookup_user(&username_input, &password_input) {
                        // use wildcard, since value has already been set as user_id
                        // in lookup_user & no longer matters
                        Ok(_) => {
                            println!();
                            println!("You are logged in!");

                            // only stops running when user wants to logout
                            run_logged_in_menu();
                            set_client_id(None); // Option can either be None or Some()
                            run_menu = false;
                            println!();
                            println!("Logout successful")

                        }
                        Err(e) => {
                            println!();
                            println!("The login attempt failed. Please try again.");
                            run_main_menu();
                        }
                    }
                    if !run_menu {
                        break;
                    }
                }
            }
           
            // will need to allocate data from the heap for a String
            let mut input = get_input();
        }
    }
    pub fn run_logged_in_menu() {
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
            println!("Enter your selection from 1-5: ");

            // will need to allocate data from the heap for a String
            let mut input = get_input();

            match input.as_str() {
                "1" => {
                    add_entry_menu();
                },
                "2" => {
                    println!("2!")
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

    pub fn add_entry_menu() {
        let mut run_options: bool = true;

        loop {
            println!();
            println!("Would you like the system to generate a password for you?");
            println!("Enter (y/n), or enter q to return to the main menu: ");

            let input = get_one_letter_input();

            match input.as_str() {
                "y" => { 
                    println!("y!")
                },
                "n" => {
                    let entry: AccountInfo = prompt_account_info_all();
                    if let Ok(_) = AccountInfo::add_account(entry) {
                        println!();
                        println!("New account successfully added!");
                        println!();
                        println!("Would you like to add another account? Enter (y/n):");
                        let input = get_one_letter_input();
                        if input == "y" {
                            add_entry_menu();
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
            let client_id: Option<u8> = get_client_id();

            if !account_name.is_empty() && !username.is_empty() && !password.is_empty() {
                println!("{}", account_name);
                println!("{}", username);
                println!("{}", password);
                entry = AccountInfo {      // entry takes ownership of all 4 values
                    account: account_name,
                    username,
                    password,
                    clientId: client_id.unwrap(), // extracts the Some value from the Option enum
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
            println!("Enter the name of your account: ");
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
            println!("Enter your account password: ");
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
}