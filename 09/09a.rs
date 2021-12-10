use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let lines = contents.lines();
	let grid: Vec<Vec<u32>> = lines.map(|line| {
		let m = line.chars().map(|c| return c.to_digit(10).unwrap()).collect();
		return m;
	}).collect();

	let mut total = 0;
	for (i, row) in grid.iter().enumerate() {
		for (j, digit) in row.iter().enumerate() {
			let mut is_low_point = true;
			if i > 0 && grid[i-1][j] <= *digit {
				is_low_point = false;
			} 
			if i < grid.len()-1 && grid[i+1][j] <= *digit {
				is_low_point = false;
			}

			if j > 0 && grid[i][j-1] <= *digit {
				is_low_point = false;
			} 
			if j < grid[0].len()-1 && grid[i][j+1] <= *digit {
				is_low_point = false;
			}
			if is_low_point {
				total += *digit+1;
			}
		}
	}
	println!("The total risk is {}", total);
}