use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let lines = contents.lines();

	let mut depth = 0;
	let mut horizontal = 0;
	let mut aim = 0;
	for line in lines {
		let vals = line.split(" ").collect::<Vec<_>>();
		let dir = vals[0];
		let amount = vals[1].parse::<i32>().unwrap();

		if "forward" == dir {
			horizontal += amount;
		} else if "backward" == dir {
			horizontal -= amount;
		} else if "up" == dir {
			aim -= amount;
		} else if "down" == dir {
			aim += amount;
		}
	}
	println!("final pos: {}, {}", horizontal, depth);
	println!("answer: {}", horizontal * depth);
}