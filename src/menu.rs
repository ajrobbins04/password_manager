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

            let input = get_input();

            // convert to &str so a read-only view can be used
            match input.as_str() {
                "1" => {

                }
                "2" => {

                }
                "3" => {

                }
                "4" => {
                    
                }
                "5" => {

                }
                "6" => {
                    
                }
                _ => {
                    // wildcard alternative
                }
            }
        }
    }
    // returns string containing user's input
    pub fn get_input() -> String  {
        // mut makes the variable mutable so the input can change
        let mut input = String::new(); 

        // passes in input as a mutable reference
        io::stdin().read_line(&mut input).expect("Failed to read line"); // .expect used for input stream error handling

        // trim removes trailing whitespace
        input.trim().to_string() // no final semicolon on lines that return data
    }
  
}
