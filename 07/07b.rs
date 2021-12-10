use std::env;
use std::fs;

fn get_mean(nums: Vec<i32>) -> i32 {
	let val = nums.iter().sum::<i32>() as f32 / nums.len() as f32;
	println!("{}", val);
	return val.floor() as i32;
}

fn distance_from_mean(nums: Vec<i32>) -> i32 {
	let point = get_mean(nums.clone());
	let mut total = 0;
	for num in nums {
		total += (1..=(num-point).abs() as i32).fold(0, |a, b| a + b);
	}
	return total as i32;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let nums: Vec<i32> = contents.lines().next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

	println!("Total fuel: {}", distance_from_mean(nums));

}