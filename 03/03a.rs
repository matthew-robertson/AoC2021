use std::env;
use std::fs;
use std::convert::TryFrom;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let lines = contents.lines();

	let parsed_lines: Vec<Vec<u32>> = lines.map(|line| {
		let m = line.chars().map(|c| return c.to_digit(10).unwrap()).collect();
		return m;
	}).collect();
	let reduction = parsed_lines.clone().into_iter().fold(vec!(0; 12), |mut sum, i| {
		for (index, value) in i.into_iter().enumerate() {
			sum[index] += value;
		}
		return sum;
	});

	let max = u32::try_from(parsed_lines.len()).unwrap();
	let gamma: String = reduction.clone().into_iter().map(|x| {
		if x*2 >= max {return "1"};
		return "0";
	}).collect();
	let epsilon: String = reduction.clone().into_iter().map(|x| {
		if x*2 >= max {return "0"};
		return "1";
	}).collect();
	println!("{}", u32::from_str_radix(&epsilon, 2).unwrap() * u32::from_str_radix(&gamma, 2).unwrap());
}