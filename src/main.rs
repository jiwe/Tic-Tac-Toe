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

fn get_move() -> (u32, u32) {
    println!("=> What is your move's X co-ordinate?:");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("x error");

    println!("=> What is your move's Y co-ordinate?:");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("y error");

    (
        x.trim().parse::<u32>().unwrap(),
        y.trim().parse::<u32>().unwrap(),
    )
}

fn make_move(
    board: Vec<Vec<char>>,
    move_coords: (u32, u32),
    player: char,
) -> Result<Vec<Vec<char>>, String> {
    let mut res = board;
    if let c = res
        .get_mut(move_coords.0 as usize)
        .unwrap()
        .get_mut(move_coords.1 as usize)
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

fn main() {
    let mut board = new_board();
    render(&board);

    let mut count = 0;
    let mut player = 'X';
    loop {
        let move_coords = get_move();
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
    }
}
