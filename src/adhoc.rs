use crate::db::establish_connection;
use crate::models::{NewBook, NewMember};
use diesel::prelude::*;
/* file in which we define functions that perform special operations
 * directly on the databases */ 

// determines whether user or book needs to be added and calls the
// appropriate function
pub fn add(conn: &PgConnection) {
    println!("Add new member or new book?");
    println!("(1) Member\t\t(2) Book");
}

fn new_book(conn: &PgConnection)) {
}

fn new_mem(conn: &PgConnection)) {
}

// determines whether user or book record needs to be modified and
// calls the appropriate function
pub fn modify(conn: &PgConnection)) {
}

fn mod_book(conn: &PgConnection)){
}

fn mod_mem(conn: &PgConnection)){
}
