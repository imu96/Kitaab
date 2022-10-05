use std::io;
mod adhoc;

fn main() {

    // Welcome message
    println!("Ya Ali Madad and welcome to the Clifton Library
    Management Software!\n");

    // String in which most recent command will be stored
    let mut cmd = String::new();

    loop {
	println!("> ");
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
