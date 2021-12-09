use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let generations = &args[2].parse::<u32>().unwrap();

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let mut lines = contents.lines();
	let starting_fish: Vec<usize> = lines.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();

	//initializing
	let mut fish_count: [u64; 9] = [0;9];
	for fish in starting_fish {
		fish_count[fish] = fish_count[fish] + 1;
	}
	println!("{:?}", fish_count);

	for _i in 0..*generations {
		let mut new_fishcount: [u64; 9] = [0;9];
		for x in 0..8 {
			new_fishcount[x] = fish_count[x+1];
		}
		new_fishcount[6] += fish_count[0];
		new_fishcount[8] += fish_count[0];

		fish_count = new_fishcount;
	}

	println!("{:?}", fish_count[0]+fish_count[1]+fish_count[2]+fish_count[3]+fish_count[4]+fish_count[5]+fish_count[6]+fish_count[7]);
}