mod menu; // Will either look for menu.rs in same directory or, if not
          // found, a directory named menu w/a mod.rs file in it.
          // Never attempt to use both options in a single project.
mod records;

fn main() {
    //menu::menu::run_menu_loop();
    
    generate_password();
}
pub fn generate_password() -> Vec<char> {
    // Define a vector containing characters for password generation
    let chars_pool: Vec<char> = (b'A'..=b'Z')
        // ..= makes the last range value inclusive
        .chain(b'a'..=b'z') 
        .chain(b'0'..=b'9')
        .chain(b'!'..=b'/')
        .chain(b':'..=b'@')
        .chain(b'['..=b'`')
        .chain(b'{'..=b'~')
        .map(|c| c as char)
        .collect();
    for c in &chars_pool {
        print!("{}", c);
    }
    chars_pool
} 