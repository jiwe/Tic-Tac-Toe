use std::io;

fn new_board() -> Vec<Vec<char>> {
    let size = 3;
    vec![vec![' '; size]; size]
}

fn render(board: Vec<Vec<char>>) {
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

    println!("=> What is your move's X co-ordinate?:");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("y error");

    (
        x.trim().parse::<u32>().unwrap(),
        y.trim().parse::<u32>().unwrap(),
    )
}

fn main() {
    let mut board = new_board();
    board[1][0] = 'X';
    board[1][1] = 'O';
    render(board);
    let move_coords = get_move();
    println!("=> {:?}", move_coords);
}
