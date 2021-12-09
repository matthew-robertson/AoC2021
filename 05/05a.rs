use std::env;
use std::fs;

fn draw_line(vent_grid: Vec<Vec<u32>>, p1: Vec<usize>, p2: Vec<usize>) -> Vec<Vec<u32>> {
	let mut new_grid = vent_grid.clone();
	if p1[0] == p2[0] {
		for y in p1[1]..(p2[1]+1) {
			new_grid[p1[0]][y] += 1;
		}
		for y in p2[1]..(p1[1]+1) {
			new_grid[p1[0]][y] += 1;
		}
	} else if p1[1] == p2[1] {
		for x in p1[0]..(p2[0]+1) {
			new_grid[x][p1[1]] += 1;
		}
		for x in p2[0]..(p1[0]+1) {
			new_grid[x][p1[1]] += 1;
		}
	}
	
	return new_grid;
}

fn count_points(vent_grid: Vec<Vec<u32>>) -> u32 {
	let mut total = 0;
	for row in vent_grid {
		for x in row {
			if x > 1 {
				total += 1;
			}
		}
	}
	return total;
}

fn print_grid(vent_grid: Vec<Vec<u32>>) {
	for row in vent_grid {
		for x in row {
			print!("{}", x);
		}
		println!("");
	}
	println!("");
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let lines = contents.lines();

	let mut vent_grid = vec![vec![0;1000]; 1000];
	for line in lines {
		let points: Vec<&str> = line.split(" -> ").collect();
		let p1: Vec<usize> = points[0].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
		let p2: Vec<usize> = points[1].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
		
		vent_grid = draw_line(vent_grid, p1, p2);
	}

	println!("Overlapping grid points: {}", count_points(vent_grid));
}