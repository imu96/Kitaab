use std::io;
use colored::Colorize;
mod adhoc;

fn main() {

    // Welcome message
    println!("Welcome to Kitaab- a library management software!");

    // String in which most recent command will be stored
    let mut cmd = String::new();

    // Options of what to do
    let options = ["lend","return","renew", "pay fines", "search",
		   "add", "edit", "quit", "help"];

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

	print!("What now>");
	io::stdin()
	    .read_line(&mut cmd)
	    .expect("Failed to read line. Please try again");
	let cmd_args : Vec<&str> = cmd.split_whitespace().collect();
	match cmd_args[0] {

	    // lends a book to a member
	    "lend" => lend(& cmd_args),
	    // returns a book 
	    "ret" => ret(& cmd_args),
	    // prints the help message
	    "help" => help(),
	    // searches for book or member
	    "search" => search(& cmd_args),
	    // adds new book or member to database
	    "add" => adhoc::add(& cmd_args),
	    // modifies entry in databases
	    "edit" => adhoc::modify(& cmd_args),
	    // pays fine for member
	    "pay" => pay(& cmd_args),
	    _ => println!("Invalid command. Please try again. Enter
	\"help\" in order to see available options"),
	}
    }
}
