use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

	let contents = fs::read_to_string(filename)
	    .expect("Something went wrong reading the file");

	let split_contents = contents.lines();
	let seats: Vec<&str> = split_contents.collect();

	let mut id_vec = vec![];

	for seat in seats {
		let row = &seat[0..7];
		let col = &seat[7..10];

		let row_bin = row.replace("B", "1");
		let row_bin = row_bin.replace("F", "0");
	    let row_int = isize::from_str_radix(&row_bin, 2).unwrap();

		let col_bin = col.replace("R", "1");
		let col_bin = col_bin.replace("L", "0");
	    let col_int = isize::from_str_radix(&col_bin, 2).unwrap();

	    let id = row_int * 8 + col_int;
	    
	    id_vec.push(id);
	}

	id_vec.sort();

	for i in 0..id_vec.len()-1 {
		if id_vec[i+1] == id_vec[i]+2 {
			println!("seat id: {}", id_vec[i]+1);
		}
	}
}
