mod menu;  // Will either look for menu.rs in same directory or, if not
           // found, a directory named menu w/a mod.rs file in it.
           // Never attempt to use both options in a single project.
mod records;
use rusqlite::{Connection, Result};
fn main() {
   menu::menu::run_main_menu();
}

/* This method only needed to be run once to created the db.
   It is being kept in case the db ever needs to be re-built. */
fn build_db() -> Result<()> {  // returns a Result tuple

    // '?' simplifies Result & Option error handling
    let conn = Connection::open("manager.db")?;  // db created if non-existent
    let clients_table = r#"
    CREATE TABLE IF NOT EXISTS clients (
        clientId INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL)
    "#;
    conn.execute(clients_table, [])?;

    let accounts_table = r#"
    CREATE TABLE IF NOT EXISTS accounts (
        accountId INTEGER PRIMARY KEY AUTOINCREMENT,
        clientId INTEGER REFERENCES clients(clientId) ON DELETE CASCADE ON UPDATE CASCADE, 
        accountName TEXT NOT NULL,
        accountUsername TEXT NOT NULL,
        accountPassword TEXT NOT NULL)

    "#; // the clientId foreign key in vault links the data so all
        // accounts for a particular client can be easily found

    conn.execute(accounts_table, [])?;

    // add two users for testing purposes during project development
    let client_values = "INSERT INTO clients (username, password)
    VALUES (\"ajrobb@byui.edu\", \"mcClapYoHands\"),
           (\"eRirie@byui.edu\", \"gusTTShowbiz\")";

    conn.execute(client_values, [])?;
    Ok(()) // '?' unwraps ok's value unless it is an error
}
