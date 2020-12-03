use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

	let contents = fs::read_to_string(filename)
	    .expect("Something went wrong reading the file");

	let split_contents = contents.lines();

	let passwords: Vec<&str> = split_contents.collect();

	let mut num_valid = 0;
	for password in passwords {
		let re = Regex::new(r"(\d*)-(\d*)\s(\w):\s(\w*)").unwrap();
		for cap in re.captures_iter(password) {
		    let first = &cap[1].parse::<usize>().unwrap();
		    let second = &cap[2].parse::<usize>().unwrap();
		    let char = &cap[3];
		    let password = &cap[4];
		    if valid(*first, *second, char, password) {
		    	num_valid += 1;
		    }
		}
	}
	println!("valid passwords: {}", num_valid);
}

/* part one
fn valid(min: usize, max: usize, char: &str, text: &str) -> bool {
	let formatted = format!(r"({})", char);
	let re = Regex::new(formatted.as_str());
	let num_caps = match re {
		Ok(regex) => {
			regex.find_iter(text).count()
		}
		Err(_) => panic!("couldn't parse regex"),
	};
	// println!("number of {}: {}", char, num_caps);
	return min <= num_caps && num_caps <= max
}
*/

fn valid(first: usize, second: usize, char: &str, text: &str) -> bool {
	let char: char = char.parse::<char>().unwrap();
	let first_char = text.as_bytes()[first-1] as char;
	let second_char = text.as_bytes()[second-1] as char;
	let valid = (first_char == char && second_char != char) || (first_char != char && second_char == char);
	println!("{} | {}({})-{}({}) {}: {}", valid, first_char, first, second_char, second, char, text);
	return valid
}
