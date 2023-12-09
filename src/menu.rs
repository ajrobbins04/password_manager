/* menu module primarily deals with interacting with the  
   user by displaying output and receiving input.     */

   

// Rust's module paths do not correspond w/the project's file paths
pub mod menu {
    use std::io; // input/output functionality from standard library
    use crate::records::records::AccountInfo; // 'crate' begins module search at root of project
    pub fn run_main_menu() {

        // exit condition for the loop
        let mut no_input: bool = true;

        loop { // loop displays output continuosly until exit condition has been met.
            println!("Password Manager Menu:"); // '!' denotes that println is a macro.
            println!("1. Add New Entry");
            println!("2. View All Entries");
            println!("3. Edit an Entry");
            println!("4. Delete an Entry");
            println!("5. Exit Password Manager");
            println!(); // space between menu and input 
            print!("Enter your selection from 1-5 > ");

            // will need to allocate data from the heap for a String
            let mut input = get_input();

            // convert to &str b/c string literals 
            // like "1" are &str data types
            match input.as_str() {
                "1" => {
                    println!("1!");
                    no_input = false;
                }
                "2" => {
                    println!("2!");
                    no_input = false;
                }
                "3" => {
                    println!("3!");
                    no_input = false;
                }
                "4" => {
                    println!("4!");
                    no_input = false;
                }
                "5" => {
                    println!("5!");
                    no_input = false;
                }
                "6" => {
                    println!("6!");
                    no_input = false;
                }
                _ => {
                    // wildcard alternative
                    println!("anything!");
                    no_input = false;
                }
            }
            // is it time to break out of the loop?
            if no_input == false {
                break;
            }
        }
    }
    pub fn add_entry_menu() {
        let mut no_input: bool = true;

        loop { // loop displays output continuosly until exit condition has been met.
            println!("Would you like the system to generate a password for you?"); 
            println!();
            print!("Enter (y/n) > ");

            let mut input = get_y_or_n();

            match input.as_str() {
                "y" => {
                    println!("y!");
                    no_input = false;
                }
                "n" => {
                    println!("n!");
                    get_all_account_info();
                }
                _ => {
                    println!("wildcard!");
                    no_input = false;
                }
            }// is it time to break out of the loop?
            if no_input == false {
                break;
            }
        }
    }

    // retrieves user input to create a new account entry
    pub fn get_all_account_info() -> AccountInfo {
        let mut entry = AccountInfo::default();
        let mut complete: bool = false;

        loop {
            let account_name = get_account_name();
            let username = get_username();
            let password = get_password();

            // make sure nothing is empty before returning the data
            if !account_name.is_empty() && !username.is_empty()
            && !password.is_empty() {
                entry = AccountInfo {
                    account: account_name,
                    username,
                    password,
                };
                complete = true;
                if complete {
                    break;
                }
            }
        } entry // return

    }
    
    // returns string containing user's input
    fn get_input() -> String  {
        // mut makes the variable mutable so the input can change
        let mut input = String::new(); 

        // passes in input as a mutable reference
        io::stdin().read_line(&mut input).expect("Failed to read line"); // .expect used for input stream error handling

        // trim converts input to &str when removing output, 
        // so to_string() must convert it back to a String
        input.trim().to_string() // exclue a semicolon to return data
    }

    // requires some manipulation to return either 'y' or 'n' 
    fn get_y_or_n() -> String {
        
        // same as get_input
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).expect("Failed to read line"); // .expect used for input stream error handling
        
        // must remove whitespace before retrieving first character from input
        input = input.trim().to_string();

        // should result in 'y' if Yes is entered or 'n' if No is entered
        input.to_lowercase().chars().nth(0).unwrap_or('\0').to_string()
    }

    fn get_account_name() -> String {

        // loses ownership too soon if defined inside loop
        let mut account_name = String::new();
        let mut account_name_empty: bool = true;
        loop {
            print!("Enter the name of your account: ");
            let mut account_name = get_input();
            if account_name.is_empty() {
                println!("ERROR: No account name entered. Please try again.");
                println!();
            }
            else {
                account_name_empty = false;
            }
            if !account_name_empty {
                break;
            }
        } account_name  // return
    }
    fn get_username() -> String {

        // loses ownership too soon if defined inside loop
        let mut username = String::new();
        let mut username_empty: bool = true;
        loop {
            print!("Enter your account username: ");
            let mut account_name = get_input();
            if account_name.is_empty() {
                println!("ERROR: No username entered. Please try again.");
                println!();
            }
            else {
                username_empty = false;
            }
            if !username_empty {
                break;
            }
        } username  // return
    }

    fn get_password() -> String {

        // loses ownership too soon if defined inside loop
        let mut password = String::new();
        let mut password_empty: bool = true;
        loop {
            print!("Enter your account password: ");
            let mut password = get_input();
            if password.is_empty() {
                println!("ERROR: No password entered. Please try again.");
                println!();
            }
            else {
                password_empty = false;
            }
            if !password_empty {
                break;
            }
        } password // return
    }
  
}
