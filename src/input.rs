use std::io::stdin;

pub fn confirm() -> bool {
    loop {
	println!("(1) yes\t\t(2) no");
	let yn= get_opt();
	if yn > 2 {
	    println!("Number entered does not correspond to an
    option. Please enter number in the given range");
	    continue;
	}
	else if yn == 1 {
	    return true;
	}
	else {
	    return false;
	}
    }
}
		

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
