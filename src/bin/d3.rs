use std::fs::File;
use std::io::*;

fn main() {
    let inp = File::open("assets/d3.txt").unwrap();
    let rdr = BufReader::new(inp);

    let board: Vec<Vec<char>> = rdr.lines().map(|l| l.unwrap().chars().collect()).collect();
    let nrow = board.len();
    let ncol = board[0].len();

    // q1
    println!("{:?}", ntrees(&board, 3, 1));

    //q2
    let ans: usize = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        .iter()
        .map(|&pair| {
            let res = ntrees(&board, pair[0], pair[1]);
            println!("{:?}", res);
            res
        })
        .product();
    println!("{:?}", ans);
}

fn ntrees(board: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut count = 0;
    let nrow = board.len();
    let ncol = board[0].len();
    let mut i = 0;
    let mut j = 0;
    while i < nrow {
        if board[i][j] == '#' {
            count += 1;
        }
        i += down;
        j = (j + right) % ncol;
    }
    count
}
