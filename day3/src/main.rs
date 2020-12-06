use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let slope = &args[1].parse::<f64>().unwrap();
	let filename = &args[2];

	let contents = fs::read_to_string(filename)
	    .expect("Something went wrong reading the file");
	let split_contents = contents.lines();
	let lines: Vec<&str> = split_contents.collect();
	
	let mut trees_hit = 0;
	let mut x: f64 = 0.0;
	for y in 0..lines.len() {
		if x % 1.0 == 0.0 && hit_tree(x as usize, y, &lines) {
			trees_hit += 1;
		}

		x += slope;
		
		let width: f64 = lines[0].len() as f64;
		if x >= width {
			x = x - width;
		}
		
	}
	println!("trees hit: {}", trees_hit);
}

fn hit_tree(x: usize, y: usize, lines: &Vec<&str>) -> bool {
	return &lines[y][x..x+1] == "#"
}
