use std::io::{self, BufRead};
use chess_engine::engine::Engine;

fn main() {
    // Create a new engine instance
    let mut engine = Engine::new("Yakari".to_string(), "EiSiMo".to_string());

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
                "position" => {
                    // Example: "position startpos moves e2e4 e7e5"
                    // Or: "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
                    // You'll need to write a parser for this!
                    // For now, let's just handle the "fen" part simply.
                    if tokens.len() > 1 && tokens[1] == "fen" {
                        let fen = tokens[2..].join(" ");
                        engine.setpos(&fen);
                    }
                }
                "go" => {
                    // Example: "go depth 6"
                    // For now, we'll just use the fixed depth from your search function.
                    engine.search(5);
                }
                "quit" => {
                    break; // Exit the loop and the program
                }
                _ => {
                    // Unknown command, just ignore
                }
            }
        }
    }

}
