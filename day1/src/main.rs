use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

	let contents = fs::read_to_string(filename)
	    .expect("Something went wrong reading the file");

	let split_contents = contents.lines();

	let nums: Vec<&str> = split_contents.collect();

	/* part 1
	for i in 0..nums.len() {
		let initial_num: i32 = nums[i].parse::<i32>().unwrap();
		for j in i..nums.len() {
			let test_num: i32 = nums[j].parse::<i32>().unwrap();
			if test_num + initial_num == 2020 {
				let result = test_num * initial_num;
				println!("{}", result);
				return
			}
		}
	}
	*/

	let length = nums.len();
	
	for i in 0..length {
		let first_num: i32 = nums[i].parse::<i32>().unwrap();
		for j in i..length {
			let second_num: i32 = nums[j].parse::<i32>().unwrap();
			for w in j..length {
				let third_num: i32 = nums[w].parse::<i32>().unwrap();
				if first_num + second_num + third_num == 2020 {
					let result = first_num * second_num * third_num;
					println!("{}", result);
					return
				}
			}
		}
	}
}
