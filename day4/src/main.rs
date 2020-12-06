use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

	let contents = fs::read_to_string(filename)
	    .expect("Something went wrong reading the file");

	let reg_vec = vec!["(byr:\\d{4})", "(iyr:\\d{4})", "(eyr:\\d{4})", "(hgt:\\d{1,3}[ci])", "(hcl:#[\\d\\w]{6})", "(ecl:\\w{3})", "(pid:\\d{9})"];
	let mut count_vec = vec![];

	for reg in reg_vec {
		let formatted = format!(r"({})", reg);
		let re = Regex::new(formatted.as_str());
		let num_caps = match re {
			Ok(regex) => {
				regex.find_iter(&contents).count()
			}
			Err(_) => panic!("couldn't parse regex"),
		};
		println!("number of {} found: {}", reg, num_caps);
		count_vec.push(num_caps);
	}

	match count_vec.iter().min() {
		Some(num) => println!("valid passports: {}", num),
	    None => println!( "'count_vec' is empty" ),
	};
}
