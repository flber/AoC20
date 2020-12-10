use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut bags = Vec::new();
    
    for line in lines {
        let parent_and_children: Vec<&str> = line.split(" contain ").collect();
        let temp_parent = parent_and_children[0];
        let mut parent = "";
        
		let descriptor = Regex::new(r"(\w+ \w+) bags{0,1}").unwrap();
		match descriptor.captures(temp_parent) {
			Some(cap) => {
				parent = cap.get(1).unwrap().as_str();
			},
			None => (),
		};

        let children_line = &parent_and_children[1][..parent_and_children[1].len()-1];
        let children: Vec<&str> = children_line.split(", ").collect();

		let mut temp_children = Vec::new();
		for child in children {
			let re = Regex::new(r"\d+ (\w+ \w+) bags{0,1}").unwrap();
			match re.captures(child) {
				Some(cap) => {
					temp_children.push(cap.get(1).unwrap().as_str())
				},
				None => (),
			};
		}
		
        let mut bag = Vec::new();
        bag.push(vec![vec![parent], temp_children]);
        bags.push(bag);
    }

	let mut check_bags: Vec<&str> = Vec::new();

	for bag in &bags {
		let children = &bag[0][1];
		if children.contains(&"shiny gold") {
			// println!("{:#?} has shiny gold", bag);
			check_bags.push(bag[0][0][0]);
		}
	}

	let mut next_check: Vec<&str> = Vec::new();

	let mut i = 0;
	while check_bags.len() > 0 {
		let check_bag = check_bags[i];
		for bag in &bags {
			let children = &bag[0][1];
			if children.contains(&check_bag) {
				// println!("{:#?} has {:#?}", bag, check_bag);
				next_check.push(bag[0][0][0]);
			}
		}
		println!("{}", next_check.len());
		if i == check_bags.len()-1 {
			check_bags.clear();
			check_bags.append(&mut next_check);
			next_check.clear();
			i = 0;
		}
		i += 1;
	}

	// println!("total bags: {}", total_bags);
}
