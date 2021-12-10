use std::env;
use std::fs;

fn get_median(mut nums: Vec<i32>) -> i32 {
	nums.sort();
	let mid = nums.len() / 2;
	return nums[mid];
}

fn get_mean(mut nums: Vec<i32>) -> i32 {
	nums.sort();
	let mid = nums.len() / 2;
	return nums[mid];
}

fn distance_from_median(nums: Vec<i32>) -> i32 {
	let median = get_median(nums.clone());
	let mut total = 0;
	for num in nums {
		total += (num - median).abs();
	}
	return total;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let nums: Vec<i32> = contents.lines().next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

	println!("Total fuel: {}", distance_from_median(nums));

}