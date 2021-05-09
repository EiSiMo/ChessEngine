use std::time::SystemTime;
use std::env;

use crate::board::Board;

mod board;
mod board_evaluation;
mod board_representation;
mod board_helpers;
mod move_searching;
mod board_init;
mod move_generation;




fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid commandline arguments");
    } else {
        let fen = args[1].as_str();
        let mut b = Board::from_fen(fen);
        b = b.minimax(6_u8, i32::MIN, i32::MAX).0;
        println!("{}", b.as_fen());
    }
    */

    let mut b = Board::default();
    let start_time = SystemTime::now();
    for _ in 0..20 {
        b = b.minimax(4_u8, i32::MIN, i32::MAX).0
    }
    match start_time.elapsed() {
        Ok(elapsed) => {
            println!("Finding the best move took {} ms", elapsed.as_millis());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

}
