use std::io::stdin;

// Gets number from user
pub fn get_num() -> u32 {
    let mut opt = String::new();
    loop { 
	stdin()
	.read_line(&mut opt)
	.expect("Failed to read line. Please try again");
	match opt.trim().parse::u32() {
	    Ok(n) => return n,
	    _ => println!("Could not be parsed as a number. Please
enter a number"),
	}
    }
}
