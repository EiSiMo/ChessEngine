// uci.rs

use std::io::{self, BufRead};
use crate::engine::Engine;

pub fn uci_mainloop(engine: &mut Engine) {
    loop {
        // Start the main UCI loop
        for line in io::stdin().lock().lines() {
            let input = line.unwrap_or_else(|_| "quit".to_string());
            let tokens: Vec<&str> = input.split_whitespace().collect();

            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "uci" => {
                    println!("id name {}", engine.name);
                    println!("id author {}", engine.author);
                    println!("uciok");
                }
                "isready" => {
                    println!("readyok");
                }
                "ucinewgame" => {
                    // not yet implemented
                }
                "position" => {
                    if tokens.len() > 1 {
                        if tokens[1] == "fen" {
                            let fen = tokens[2..].join(" ");
                            engine.setpos_fen(&fen);
                        } else if tokens[1] == "startpos" {
                            if tokens.len() > 2 && tokens[2] == "moves" {
                                engine.setpos_startpos(&tokens[3..]);
                            } else {
                                engine.setpos_startpos(&[]);
                            }
                        }
                    }
                }
                "go" => {
                    // TODO add a lot functionality
                    println!("{}", engine.search(6));
                }
                "stop" => {
                    // TODO stop search as soon as possible
                }
                "quit" => {
                    return;
                }
                _ => {
                    // Unknown command, just ignore
                }
            }
        }
    }
}