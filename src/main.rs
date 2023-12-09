mod menu; // Will either look for menu.rs in same directory or, if not
          // found, a directory named menu w/a mod.rs file in it.
          // Never attempt to use both options in a single project.
mod records;

fn main() {
    menu::menu::run_menu_loop();
}
