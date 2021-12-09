use std::str::Lines;
use std::env;
use std::fs;

#[derive(Clone)]
struct Square {
	val: u32,
	is_called: bool,
}

#[derive(Clone)]
struct Board {
	winner: bool,
	board: Vec<Square>
}

/**
 *  0, 1, 2, 3, 4
 *  5, 6, 7, 8, 9
 * 10,11,12,13,14
 * 15,16,17,18,19
 * 20,21,22,23,24
 **/
fn apply_number(number: u32, board: &mut Board) -> Vec<Square> {
	let mut s = vec!();
	for square in board.board.to_vec() {
		let mut is_called = square.is_called;
		if square.val == number {
			is_called = true;
		}
		s.push(Square{ val: square.val, is_called: is_called});
	}
	return s;
}

fn check_win(board: Vec<Square>) -> bool {
	// Check rows
	for i in 0..5 {
		let offset = i*5;
		if
			board[offset+0].is_called &&
			board[offset+1].is_called &&
			board[offset+2].is_called &&
			board[offset+3].is_called &&
			board[offset+4].is_called
			{
			return true;
		}
	}

	// Check Columns
	for i in 0..5 {
		if 
			board[i].is_called &&
			board[i+5].is_called &&
			board[i+10].is_called &&
			board[i+15].is_called &&
			board[i+20].is_called
			 {
			return true;
		}
	}

	// No bingo
	return false;
}

fn print_board(board: &Board) {
	let mut c = 1;
	for square in board.board.to_vec() {
		print!("{}", square.val);
		if square.is_called {
			print!("t,");
		} else {
			print!("f,");
		}
		if c % 5 == 0 {
			println!("");
		}

		c = c+1;
	}
	println!("");

}

fn print_score(number: u32, board: &Board) {
	println!("{}",number);
	let mut score = 0;
	for square in board.board.to_vec() {
		if !square.is_called {
			score += square.val;
		}
	}
	println!("{}", score * number);
	println!("WINNER, GANGANT");
}

fn process_call(number: u32, boards: &mut Vec<Board>) -> bool {
	let mut is_winner = false;
	for board in boards {
		board.board = apply_number(number, board);
		let t_winner = check_win(board.board.to_vec());
		if t_winner {
			if !board.winner {
				print_score(number, board);
			}
			board.winner = true;
			is_winner = true;
		}
	}
	return is_winner;
}

fn get_boards(mut lines: Lines) -> Vec<Board> {
	let mut boards = vec!();
	while let (Some(_newline), Some(l1), Some(l2), Some(l3), Some(l4), Some(l5)) = (lines.next(), lines.next(), lines.next(), lines.next(), lines.next(), lines.next()) {
		let mut board = vec!();
		for ln in vec!(l1, l2, l3, l4, l5) {
			for l in ln.split(" ") {
				if l.is_empty() { continue; }
				board.push(Square { val: l.parse::<u32>().unwrap(), is_called: false });
			}
		}
		boards.push(Board{ winner: false, board: board });
	}
	return boards;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
	let mut lines = contents.lines();
	let calls: Vec<u32> = lines.next().unwrap().split(",").map(|x| x.parse::<u32>().unwrap()).collect();
	let mut boards = get_boards(lines);

	for call in calls.clone() {
		let result = process_call(call, &mut boards);
		let mut c = false;
		for board in boards.clone() {
			if board.winner == false {
				c = true;
				break;
			}

		}
		if !c{
			break;
		}
	}
}