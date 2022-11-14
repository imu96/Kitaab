use crate::models::{Member, NewIssue, Book};
use diesel::prelude::*;
use crate::input::get_opt;
use std::io;

pub mod conxn; 
pub mod models;
pub mod schema;

// Ensures valid member ID is entered, and returns it
fn get_user(conn: &PgConnection) -> u32 {
    use self::schema::members::dsl::*;
    // Grabs a member ID until a valid one is provided
    println!("Please enter a member ID ");
    loop {
	print!(">");
	// Checks if user ID corresponds to existing user. If not, asks
	// user to input ID again
	let mem = members.find(get_num()).first(conn);
	if let Error(_) = mem {
	    println!("ID does not exist. Please enter valid member
   ID:");
	    continue;
	}
	let recrd = mem.unwrap();
	println!("Issue book to {} {}?", recrd.fst_name,
		 recrd.lst_name);
	if !confirm() {
	    continue;
	}
	// TODO: Check if user has already taken out a book
	// TODO: Check if member's balance is paid
    }
}

// TODO: Get book to be issued

/* Creates a record in the issues database to issue
   a book to a user */
pub fn iss(conn: &PgConnection) {
    use self::schema::issues::dsl::*;
    use self::schema::members::dsl::*;
    use self::schema::books::dsl::*;
    let mem_id = get_user(conn);
    let book_id = get_book(conn);
    // TODO: Update record in books database
    // TODO: Update record in members database
    // TODO: Create record for loans
}
