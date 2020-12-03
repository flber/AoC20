use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

	let contents = fs::read_to_string(filename)
	    .expect("Something went wrong reading the file");

	let split_contents = contents.lines();

	let lines: Vec<&str> = split_contents.collect();

	let mut trees_hit = 0;
	for line in lines {
		
	}
	println!("trees hit: {}", trees_hit);
}
