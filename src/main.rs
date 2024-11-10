fn new_board() -> Vec<Vec<char>> {
    vec![vec![' '; 3]; 3]
}

fn main() {
    println!("{:?}", new_board());
}
