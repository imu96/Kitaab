use std::io::stdin;

// Gets option choice from user
pub fn get_opt() -> String {
    let mut opt = String::new();
    stdin()
    .read_line(&mut opt)
    .expect("Failed to read line. Please try again");
    opt.trim()
}
