#[macro_use]
extern crate phf;

mod evaluation;
mod move_search;
mod uci_interface;
mod engine;

use crate::engine::engine::Engine;

use chess::Board;


fn main() {
    let mut erbsenhirn: Engine = Engine {
        name: String::from("Erbsenhirn"),
        author: String::from("Moritz"),
        position: Board::default(),
        quit: false
    };
    erbsenhirn.uci_loop();
}
