// Rust's module paths do not correspond w/the project's file paths
pub mod menu {

    use std::io; // import input/output functionality from standard library

    pub fn run_menu_loop() {

        // exit condition for the loop
        let mut no_input: bool = true;

        loop { // loop displays output continuosly until exit condition has been met.
            println!("Password Manager Menu:"); // '!' denotes that println is a macro.
            println!("1. Add New Entry");
            println!("2. View All Entries");
            println!("3. Search for Entry");
            println!("4. Edit an Entry");
            println!("5. Delete an Entry");
            println!("6. Exit Password Manager");
            println!(); // space between menu and input 
            println!("Enter your selection from 1-6:");

            // will need to allocate data from the heap for a String
            let input = get_input();

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
    // returns string containing user's input
    pub fn get_input() -> String  {
        // mut makes the variable mutable so the input can change
        let mut input = String::new(); 

        // passes in input as a mutable reference
        io::stdin().read_line(&mut input).expect("Failed to read line"); // .expect used for input stream error handling

        // trim converts input to &str when removing output, 
        // so to_string() must convert it back to a String
        input.trim().to_string() // exclue a semicolon to return data
    }
  
}
