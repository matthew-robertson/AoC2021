use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let lines = contents.lines();

	let mut total = 0;
	for line in lines {
		let mut segment_map = HashMap::new();
		let mut score_map = HashMap::new();
		score_map.insert(42, 0);
		score_map.insert(17, 1);
		score_map.insert(34, 2);
		score_map.insert(39, 3);
		score_map.insert(30, 4);
		score_map.insert(37, 5);
		score_map.insert(41, 6);
		score_map.insert(25, 7);
		score_map.insert(49, 8);
		score_map.insert(45, 9);

		let inputs = line.split(" | ").collect::<Vec<&str>>()[0];
		for c in ['a','b','c','d','e','f','g'].iter() {
			segment_map.insert(c, inputs.chars().filter(|x| x == c).count());
		}

		let outputs = line.split(" | ").collect::<Vec<&str>>()[1];
		let mut temp_total = 0;
		for output in outputs.split(" ").collect::<Vec<&str>>() {
			let mut t2 = 0;
			for c in output.chars() {
				t2 += segment_map.get(&c).unwrap();
			}
			temp_total *= 10;
			temp_total += score_map.get(&t2).unwrap()
		}
		total += temp_total;
	}

	println!("Sum of outputs: {}", total);
}