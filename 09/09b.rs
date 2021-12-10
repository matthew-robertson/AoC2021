use std::env;
use std::fs;

fn get_low_points(grid: Vec<Vec<u32>>) -> Vec<Vec<usize>> {
	let mut low_points = vec!();
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
				low_points.push(vec!(i,j));
			}
		}
	}
	return low_points;
}

fn get_basin_sizes(grid: Vec<Vec<u32>>, low_points: Vec<Vec<usize>>) -> Vec<usize> {
	let mut basins = vec!();
	for low_point in low_points {
		basins.push(expand_basin(grid.clone(), low_point, vec!(), 0).len());
	}
	return basins;
}

fn expand_basin(grid: Vec<Vec<u32>>, new_element: Vec<usize>, current_basin: Vec<Vec<usize>>, depth: u32) -> Vec<Vec<usize>> {
	let i = new_element[0];
	let j = new_element[1];
	if current_basin.contains(&new_element) {
		return current_basin;
	}

	let mut expanded = current_basin.clone();
	expanded.push(new_element);
	let mut new_basin = expanded.clone();
	for point in vec!(vec!(i.saturating_sub(1), j), vec!(i+1, j), vec!(i, j.saturating_sub(1)), vec!(i, j+1)) {
		if point[0] <= 0 || point[0] >= grid.len()-1 ||
			point[1] <= 0 || point[1] >= grid[0].len()-1 {
			continue;
		}
		if grid[point[0]][point[1]] < 9 {
			new_basin = expand_basin(grid.clone(), vec!(point[0],point[1]), expanded, depth+1);
			expanded = new_basin.clone();
		}
	}
	new_basin.sort();
	new_basin.dedup();
	return new_basin;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let lines = contents.lines();
	let grid: Vec<Vec<u32>> = lines.map(|line| {
		let m = line.chars().map(|c| return c.to_digit(10).unwrap()).collect();
		return m;
	}).collect();

	let low_points = get_low_points(grid.clone());
	let mut basin_sizes = get_basin_sizes(grid.clone(), low_points);
	basin_sizes.sort();
	basin_sizes.reverse();

	println!("Multiplied 3 larges basins: {}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}