use std::io;
use using_libc::read_dir;

fn main() {
    println!("Hello, libc! Enter dir path:");
    let stdin = io::stdin();
    let mut user_input = String::new();
    if let Ok(_) = stdin.read_line(&mut user_input) {
		let trimmed_input = user_input.trim();
		let result = read_dir(trimmed_input);
		match result {
			Ok(names) => {
				println!("File names:");
				for name in names {
					println!("{:?}", name);
				}
			},
			Err(error) => {
				println!("{}", error);
			}
		}
		
	} else {
		println!("Something went wrong with input.");
	}
}
