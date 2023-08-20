use rand::prelude::*;
use std::{thread::sleep, time::Duration};

use clap::Parser;

type Frame = Vec<Vec<i32>>;

pub struct Board {
    items: Frame,
    rows: usize,
    cols: usize,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long, default_value_t = 20)]
    cols: usize,

    /// Number of times to greet
    #[arg(long, default_value_t = 20)]
    rows: usize,
}

const ALIVE: &str = "@";
const DEAD: &str = " ";

pub fn gen_board(num_rows: usize, num_cols: usize) -> Board {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut items: Vec<Vec<i32>> = Vec::new();
    for _ in 0..num_rows {
        let mut col: Vec<i32> = Vec::new();
        for _ in 0..num_cols {
            let alive: f64 = rng.gen::<f64>();
            if alive > 0.5 {
                col.push(1);
            } else {
                col.push(0)
            }
        }
        items.push(col);
    }

    Board {
        items: items,
        rows: num_rows,
        cols: num_cols,
    }
}

fn update(board: &Board) -> Frame {
    let mut next_gen: Vec<Vec<i32>> = board.items.clone();
    for i in 0..board.rows {
        for j in 0..board.cols {
            let alive: bool = board.items[i][j] > 0;
            let mut neigbors: i32 = 0;
            // Iterate through every neighbors
            for x in -1i32..=1 {
                for y in -1i32..=1 {
                    // Position of one of the neighbors (new_x, new_y)
                    let our_x: i32 = (i as i32) + x;
                    let our_y: i32 = (j as i32) + y;

                    // Make sure the position is within the bounds of the grid
                    if our_x > 0
                        && our_y > 0
                        && our_x < board.rows as i32
                        && our_y < board.cols as i32
                    {
                        neigbors += board.items[our_x as usize][our_y as usize];
                    }
                }
            }

            // Substract the state of the current cell to get the number of alive neighbors
            if alive {
                neigbors -= 1;
            }

            // Apply the rules
            if alive && neigbors < 2 {
                next_gen[i][j] = 0;
            } else if alive && neigbors > 3 {
                next_gen[i][j] = 0;
            } else if !alive && neigbors == 3 {
                next_gen[i][j] = 1;
            } else {
                next_gen[i][j] = alive as i32;
            }
        }
    }

    next_gen
}

// fn get_neighbors(board: &Board, row: usize, col: usize) -> u32 {
//     let rows = board.rows as i32;
//     let cols = board.cols as i32;
//     let our_row = i32::try_from(row).unwrap();
//     let our_col = i32::try_from(col).unwrap();
//     let mut count = 0;
//     for (i, j) in POSITIONS {
//         let next_x = our_row + i;
//         let next_y = our_col + j;
//         if next_x > 0 && next_x < rows && next_y > 0 && next_y < cols {
//             if board.items[next_x as usize][next_y as usize] == ALIVE {
//                 count += 1;
//             }
//         }
//     }
//     count
// }

// fn is_alive(board: &Board, row: usize, col: usize) -> bool {
//     return (row < board.rows) && (col < board.cols) && board.items[row][col] == ALIVE;
// }

fn print_board(board: &Board) {
    print!(" ");
    for _ in 0..board.cols - 1 {
        print!("-")
    }
    println!(" ");

    for row in board.items.clone() {
        print!("|");
        for col in row {
            if col > 0 {
                print!("{}", ALIVE);
            } else {
                print!("{}", DEAD);
            }
        }
        print!("|");
        println!();
    }

    print!(" ");
    for _ in 0..board.cols - 1 {
        print!("-")
    }
    println!(" ");
}

fn main() {
    let args: Args = Args::parse();

    let mut board: Board = gen_board(args.rows, args.cols);

    loop {
        print_board(&board);
        board.items = update(&board);
        sleep(Duration::from_millis(100));
    }
}
