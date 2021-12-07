use std::fs;
use std::process;

#[derive(Clone)]
#[derive(Debug)]
struct CalledNum {
    n: i32,
    mark: bool,
}

struct Board {
    columns: Vec<Vec<CalledNum>>,
    rows: Vec<Vec<CalledNum>>
}

fn create_board() -> Board {
    return Board {
        columns: Vec::new(),
        rows: Vec::new(),
    };
}

fn add_to_current_board(board: &mut Board, line: &str) {
    println!("{}", line);
    let row = line.trim().split_whitespace().map(|x| x.parse().expect("not a number!"));
    board.rows.push(row.clone().map(|x| CalledNum {n:x, mark:false,}).collect());

    for (pos,entry) in row.enumerate() {
        if board.columns.len() <= pos {
            board.columns.resize(pos+1, Vec::new());
        }
        board.columns[pos].push(CalledNum {
            n: entry,
            mark: false,
        });
    } 
}

fn mark_number(board: &mut Board, n: i32) {
    for r in 0..board.rows.len() {
        for c in 0..board.rows[r].len() {
            if n == board.rows[r][c].n {
                board.rows[r][c].mark = true;
            }
        }
    }
    for c in 0..board.columns.len() {
        for r in 0..board.columns[c].len() {
            if n == board.columns[c][r].n {
                board.columns[c][r].mark = true;
            }
        }
    }
}

fn is_winner(board: &Board) -> bool {
    for c in &board.columns {
        if c.iter().filter(|x| x.mark == false).count() == 0 {
            return true;
        }
    }

    for r in &board.rows {
        if r.iter().filter(|x| x.mark == false).count() == 0 {
            return true;
        }
    }

    return false;
}

fn sum_unmarked(board: &Board) -> i32 {
    let mut sum: i32 = 0;
    for c in &board.columns {
        sum = c.iter().filter(|x| x.mark == false).fold(sum, |sum, x| sum + x.n);
    }
    return sum;
}



fn main() {
    let data = fs::read_to_string("sample.txt").expect("Unable to read file");
    let numbers: Vec<i32> = data.lines().next().unwrap().split(',').map(|x| x.parse().expect("not a number!")).collect();
    
    let mut current_board: Board = create_board();
    let mut boards: Vec<Board> = Vec::new();
    for board in data.lines().skip(1) {
        if board.trim().len() > 0 {
            add_to_current_board(&mut current_board, board);
        }
        else {
            boards.push(current_board);
            current_board = create_board();
            println!("END OF BOARD");
        }
    }
    boards.push(current_board);

    for n in numbers {
        for mut b in &mut boards {
            if is_winner(&b) {
                continue;
            }
            mark_number(&mut b, n);
            if is_winner(&b) {
                let s = sum_unmarked(&b);
                println!("Winning board! Number: {}, sum: {}, total: {}", n, s, n*s);
            }
        }
    }
}
