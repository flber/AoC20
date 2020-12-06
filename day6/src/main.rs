use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

	let contents = fs::read_to_string(filename)
	    .expect("Something went wrong reading the file");

	let split_contents = contents.lines();
	let answers: Vec<&str> = split_contents.collect();

	let mut groups = vec![vec!["";0];1];
	let mut num_people = vec![0;1];
	
	for answer in answers {
		if answer != "" {
			let mut person: Vec<&str> = answer.split("").collect();
			person.remove(0);
			person.remove(person.len()-1);
			let group = groups.len()-1;
			groups[group].append(&mut person);
			groups[group].sort();

			num_people[group] += 1;
		} else {
			groups.push(vec!["";0]);
			num_people.push(0);
		}
	}

	let mut sum = 0;

	for i in 0..groups.len() {
		let group = &groups[i];
		let mut all_answers = vec!["";0];
		for answer in group {
			if group.iter().filter(|&n| &*n == answer).count() == num_people[i] {
				all_answers.push(answer);
			}
		}
		all_answers.sort();
		all_answers.dedup();
		sum += all_answers.len();
	}

	println!("sum: {}", sum);
}
