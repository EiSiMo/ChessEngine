#[macro_use]
extern crate phf;

mod evaluation;
mod move_search;
mod uci_interface;
mod engine;

use std::time::{Instant, Duration};

use crate::engine::engine::Engine;

use chess::Board;


fn benchmark_engine(engine: &mut Engine) {
    println!("Benchmarking engine...");
    let start = Instant::now();
    let board = Board::default();
    let mut counter = 0;
    loop {
        counter += 1;

        if counter % 10_000 == 0 {
            let now = Instant::now();
            if now.duration_since(start) >= Duration::new(10, 0) {
                break;
            }
        }
        engine.evaluate(board);
    }

    println!("\tEVALUATION: {}", counter);


    let start = Instant::now();
    let board = Board::default();
    let mut counter = 0;
    loop {
        counter += 1;

        if counter % 1 == 0 {
            let now = Instant::now();
            if now.duration_since(start) >= Duration::new(10, 0) {
                break;
            }
        }
        engine.minmax(board, 4);
    }
    println!("\tBEST MOVE:  {}", counter);

}


fn main() {
    let mut erbsenhirn: Engine = Engine {
        name: String::from("Erbsenhirn"),
        author: String::from("Moritz"),
        position: Board::default()
    };
    benchmark_engine(&mut erbsenhirn);
    erbsenhirn.uci_loop();
}
