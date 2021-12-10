use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let lines = contents.lines();

	let mut total = 0;
	for line in lines {
		let outputs = line.split(" | ").collect::<Vec<&str>>()[1];
		for output in outputs.split(" ").collect::<Vec<&str>>() {
			if output.len() == 2 { total += 1; }
			if output.len() == 3 { total += 1; }
			if output.len() == 4 { total += 1; }
			if output.len() == 7 { total += 1; }
		}
	}
	println!("Total number of 1,4,7, and 8s: {}", total);
}