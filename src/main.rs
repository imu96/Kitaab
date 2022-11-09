use colored::Colorize;
use crate::input::get_opt;

pub mod adhoc;
pub mod ops;
pub mod models;
pub mod schema;

fn main() {

    // Welcome message
    println!("Welcome to Kitaab- a library management software!");

    // Options of what to do
    let options = ["issue","return", "pay", "search",
		   "add", "edit", "quit", "help"];
    // Functions corresponding to each option
    let fns : Vec<fn()> = vec![iss, ret, pay, search, add,
			       edit, quit, help];

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
	let trimmed = get_opt();
	match trimmed.parse::u32() {
	    Ok(i) if i < fns.len() => fns[i](),
	    Ok(i) => println!("Entered number is not an option. Please
	try again. "),
	    _ => println!("Enter a valid number, please."),
	}
    }
}
