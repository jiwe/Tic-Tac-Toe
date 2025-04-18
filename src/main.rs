use rand::Rng;
use std::{borrow, io};

fn new_board() -> Vec<Vec<char>> {
    let size = 3;
    vec![vec![' '; size]; size]
}

fn render(board: &Vec<Vec<char>>) {
    let width = board[0].len();
    let height = board.len();

    for (i, r) in board.iter().enumerate() {
        if i == 0 {
            print!("  ");
            for j in 0..width {
                print!("{} ", j);
            }
            println!();
            print!("  ");
            for _ in 0..width {
                print!("--");
            }
            println!();
        }

        for (j, n) in r.iter().enumerate() {
            if j == 0 {
                print!("{}|", i);
            }

            print!("{} ", n);

            if j == width - 1 {
                println!("|");
            }
        }

        if i == height - 1 {
            print!("  ");
            for _ in 0..width {
                print!("--");
            }
            println!();
        }
    }
}

fn get_move() -> (usize, usize) {
    println!("=> What is your move's X co-ordinate?:");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("x error");

    println!("=> What is your move's Y co-ordinate?:");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("y error");

    (
        x.trim().parse::<usize>().unwrap(),
        y.trim().parse::<usize>().unwrap(),
    )
}

fn random_ai(board: &Vec<Vec<char>>, current_player: char) -> (usize, usize) {
    let num_rows = board.len();
    let num_cols = board[0].len();
    let mut rng = rand::thread_rng();
    loop {
        let (i, j) = (rng.gen_range(0..num_rows), rng.gen_range(0..num_cols));
        if board[i][j] == ' ' {
            return (i, j);
        }
    }
}

fn find_empty_index(board: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    let num_rows = board.len();
    let num_cols = board[0].len();
    let mut combined = board.clone();
    for j in 0..num_cols {
        combined.push((0..num_rows).map(|row_index| board[row_index][j]).collect());
    }
    combined.push(
        (0..num_rows)
            .map(|row_index| board[row_index][row_index])
            .collect(),
    );
    combined.push(
        (0..num_rows)
            .map(|row_index| board[row_index][2 - row_index])
            .collect(),
    );

    for (i, row) in combined.iter().enumerate() {
        let mut count = 0;
        let mut empty_index = None;

        for (j, &val) in row.iter().enumerate() {
            if val == target {
                count += 1;
            } else if val == ' ' {
                empty_index = Some((i, j));
            }
        }

        if count == 2 && empty_index.is_some() {
            if (3..6).contains(&i) {
                let (x, y) = empty_index.unwrap();
                empty_index = Some((y, x - 3));
            } else if i == 6 {
                let (_x, y) = empty_index.unwrap();
                empty_index = Some((y, y));
            } else if i == 7 {
                let (_x, y) = empty_index.unwrap();
                empty_index = Some((y, 2 - y));
            }
            return empty_index;
        }
    }

    None
}

fn finds_winning_moves_ai(board: &Vec<Vec<char>>, current_player: char) -> (usize, usize) {
    if let Some(index) = find_empty_index(board, current_player) {
        index
    } else {
        random_ai(board, current_player)
    }
}

fn finds_winning_and_losing_moves_ai(
    board: &Vec<Vec<char>>,
    current_player: char,
) -> (usize, usize) {
    if let Some(index) = find_empty_index(board, current_player) {
        index
    } else {
        let opponent_player = if current_player == 'O' { 'X' } else { 'O' };
        if let Some(index) = find_empty_index(board, opponent_player) {
            index
        } else {
            random_ai(board, current_player)
        }
    }
}

fn make_move(
    board: Vec<Vec<char>>,
    move_coords: (usize, usize),
    player: char,
) -> Result<Vec<Vec<char>>, String> {
    let mut res = board;
    if let c = res
        .get_mut(move_coords.0)
        .unwrap()
        .get_mut(move_coords.1)
        .unwrap()
    {
        if *c == ' ' {
            *c = player;
            Ok(res)
        } else {
            Err("err1".to_string())
        }
    } else {
        Err("err2".to_string())
    }
}

fn get_winner(board: &Vec<Vec<char>>) -> Option<char> {
    let num_rows = board.len();
    let num_cols = board[0].len();
    let mut combined = board.clone();
    for j in 0..num_cols {
        combined.push((0..num_rows).map(|row_index| board[row_index][j]).collect());
    }
    combined.push(
        (0..num_rows)
            .map(|row_index| board[row_index][row_index])
            .collect(),
    );
    combined.push(
        (0..num_rows)
            .map(|row_index| board[row_index][2 - row_index])
            .collect(),
    );

    for line in combined {
        let players = vec!['X', 'O'];
        for player in players {
            if line.iter().all(|&x| x == player) {
                return Some(player);
            }
        }
    }

    None
}

fn is_board_full(board: &Vec<Vec<char>>) -> bool {
    for line in board {
        if line.iter().any(|&x| x == ' ') {
            return false;
        }
    }
    true
}

fn main() {
    let mut board = new_board();
    render(&board);

    let mut count = 0;
    let mut player = 'X';
    loop {
        // let move_coords = get_move();
        // let move_coords = random_ai(&board, player);
        // let move_coords = finds_winning_moves_ai(&board, player);
        let move_coords = finds_winning_and_losing_moves_ai(&board, player);
        println!("{} => {:?}", player, move_coords);
        if count % 2 == 0 {
            player = 'O';
        } else {
            player = 'X';
        }
        board = make_move(board.clone(), move_coords, player).unwrap();
        render(&board);
        if let Some(p) = get_winner(&board) {
            println!("Winner: {}", p);
            break;
        }
        count += 1;

        if is_board_full(&board) {
            println!("IT'S A DRAW!!");
            break;
        }
    }
}
