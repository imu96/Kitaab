use std::io;
use colored::Colorize;
mod adhoc;

pub mod models;
pub mod schema;

fn main() {

    // Welcome message
    println!("Welcome to Kitaab- a library management software!");

    // String in which most recent command will be stored
    let mut cmd = String::new();

    // Options of what to do
    let options = ["issue / renew","return", "pay", "search",
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
	    print!("{}: {.1}{} \t \t",i,
		   format!(options[i]).bold().blue(),
		   options[i][1..]);
	}

	print!("Enter option number>");
	io::stdin()
	    .read_line(&mut cmd)
	    .expect("Failed to read line. Please try again");
	let trimmed = cmd.trim();
	match trimmed.parse::u32() {
	    Ok(i) if i < fns.len() => fns[i](),
	    Ok(i) => println!("Entered number is not an option. Please
	try again. "),
	    _ => println!("Enter a valid number, please."),
	}
    }
}
