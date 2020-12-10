use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let passports: Vec<&str> = contents.as_str().split("\n\n").collect();
    
    let mut num_valid = 0;

    for passport in passports {
        let pass_lines: Vec<&str> = passport.split("\n").collect();
        let mut pass_entries: Vec<&str> = vec!["";0];
        
        for pass_line in pass_lines {
        	pass_entries.append(&mut pass_line.split(" ").collect());
        }

       	if valid(pass_entries) {
       		num_valid += 1;
       	}
    }

    println!("num valid: {}", num_valid);
}

fn valid(entries: Vec<&str>) -> bool {
	let mut valid_entries = 0;
	
	for entry in entries {
		if entry.len() > 4 {
			let entry_type = &entry[..3];
			let entry_value = &entry[4..];

			match entry_type {
				"byr" => {
					let val = match entry_value.parse::<i32>(){
						Ok(num) => num,
						Err(_) => -1,
					};
					if 1920 <= val && val <= 2002 {
						valid_entries += 1;
					}
				},
				"iyr" => {
					let val = match entry_value.parse::<i32>(){
						Ok(num) => num,
						Err(_) => -1,
					};
					if 2010 <= val && val <= 2020 {
						valid_entries += 1;
					}
				},
				"eyr" => {
					let val = match entry_value.parse::<i32>(){
						Ok(num) => num,
						Err(_) => -1,
					};
					if 2020 <= val && val <= 2030 {
						valid_entries += 1;
					}
				},
				"hgt" => {
					let meas = &entry_value[entry_value.len()-2..];
					if meas == "cm" {
						let hgt = match entry_value[..3].parse::<i32>(){
							Ok(num) => num,
							Err(_) => -1,
						};
						if 150 <= hgt && hgt <= 193 {
							valid_entries += 1;
						}
					} else if meas == "in" {
						let hgt = match entry_value[..2].parse::<i32>(){
							Ok(num) => num,
							Err(_) => -1,
						};
						if 59 <= hgt && hgt <= 76 {
							valid_entries += 1;
						}
					}
				},
				"hcl" => {
					let re = Regex::new(r"(#[a-z0-9]{6})").unwrap();
					let num_caps = match re.captures(entry_value) {
						Some(caps) => caps.len(),
						None => 0,
					};
					if num_caps > 0 {
						valid_entries += 1;
					}
				},
				"ecl" => {
					let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
					if colors.contains(&entry_value){
						valid_entries += 1;
					}
				},
				"pid" => {
					if entry_value.len() == 9 {
						match entry_value.parse::<u32>() {
							Ok(_) => valid_entries += 1,
							Err(_) => (),
						};
					}
				},
				_ => ()
			}
		}
	}

	return valid_entries == 7
}
