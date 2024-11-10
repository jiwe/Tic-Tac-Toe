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

fn main() {
    let mut board = new_board();
    board[1][0] = 'X';
    board[1][1] = 'O';
    render(board);
}
