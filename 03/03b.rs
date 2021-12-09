use std::env;
use std::fs;
use std::convert::TryFrom;

fn get_common(length: usize, elements: Vec<Vec<u32>>) -> Vec<u32> {
	let reduction = elements.clone().into_iter().fold(vec!(0; length), |mut sum, i| {
		for (index, value) in i.into_iter().enumerate() {
			sum[index] += value;
		}
		return sum;
	});
	let max = u32::try_from(elements.len()).unwrap();
	return reduction.clone().into_iter().map(|x| {
		if x*2 >= max {return 1};
		return 0;
	}).collect();
}

fn get_uncommon(length: usize, elements: Vec<Vec<u32>>) -> Vec<u32> {
	let reduction = elements.clone().into_iter().fold(vec!(0; length), |mut sum, i| {
		for (index, value) in i.into_iter().enumerate() {
			sum[index] += value;
		}
		return sum;
	});
	let max = u32::try_from(elements.len()).unwrap();
	return reduction.clone().into_iter().map(|x| {
		if x*2 >= max {return 0};
		return 1;
	}).collect();
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let lines = contents.lines();
	let line_length = 12;

	let parsed_lines: Vec<Vec<u32>> = lines.map(|line| {
		let m = line.chars().map(|c| return c.to_digit(10).unwrap()).collect();
		return m;
	}).collect();

	let mut oxy = parsed_lines.clone();
	let mut co2 = parsed_lines.clone();
	for x in 0..line_length {
		let common: Vec<u32> = get_common(line_length, oxy.clone());
		let uncommon: Vec<u32> = get_uncommon(line_length, co2.clone());
		if oxy.len() > 1 {
			oxy = oxy.into_iter().filter(|l| l[x] == common[x]).collect();
		}
		if co2.len() > 1 {
			co2 = co2.into_iter().filter(|l| l[x] == uncommon[x]).collect();
		}
	}
	println!("{:?}, {:?}", oxy[0], co2[0]);
	let fino: String = oxy[0].clone().into_iter().map(|x| {
		if x == 1 {return "1"};
		return "0";
	}).collect();
	let finc: String = co2[0].clone().into_iter().map(|x| {
		if x == 1 {return "1"};
		return "0";
	}).collect();
	println!(
		"{}, {}, {}",
		u32::from_str_radix(&fino, 2).unwrap(),
		u32::from_str_radix(&finc, 2).unwrap(),
		u32::from_str_radix(&fino, 2).unwrap() * u32::from_str_radix(&finc, 2).unwrap());
}