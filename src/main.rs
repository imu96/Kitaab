use colored::Colorize;
use crate::input::get_opt;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod adhoc;
pub mod ops;
pub mod models;
pub mod schema;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    // Welcome message
    println!("Welcome to Kitaab- a library management software!");

    // Options of what to do
    let options = ["issue","return", "pay", "search",
		   "add", "edit", "quit", "help"];
    // Functions corresponding to each option
    let fns : Vec<fn()> = vec![iss, ret, pay, search, add,
			       edit, quit, help];

    // Opens a connection to the database
    let conn = establish_connection();
    loop {
	println!("{}", "*** Commands ***".bold());

	for i in 0..options.len() {
	    // Prints new line if four options are on the line
	    if i % 4 == 0 {
		println!("");
	    }
	    print!("({}): {}\t\t", format!(i).bold().blue(),
		   options[i]);
	}

	print!("Enter option number>");
	let opt = get_opt();
	if opt < fns.len {
	    fns[i](&conn);
	}
	else {
	    println!("Entered number is not an option. Please
	try again.");
	}
    }
}
