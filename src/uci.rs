use std::io::{self, BufRead};
use crate::engine::Engine;

pub struct UCI {
    // TODO lifetime specifier
    pub engine: Engine,
}

impl UCI {
    pub fn uci_mainloop(&mut self) {
        loop {
            for line in io::stdin().lock().lines() {
                let input = line.unwrap_or_else(|_| "quit".to_string());
                let tokens: Vec<&str> = input.split_whitespace().collect();

                if tokens.is_empty() {
                    continue;
                }

                match tokens[0] {
                    "uci" => {
                        println!("id name {}", self.engine.name);
                        println!("id author {}", self.engine.author);
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
                                self.engine.setpos_fen(&fen);
                            } else if tokens[1] == "startpos" {
                                if tokens.len() > 2 && tokens[2] == "moves" {
                                    self.engine.setpos_startpos(&tokens[3..]);
                                } else {
                                    self.engine.setpos_startpos(&[]);
                                }
                            }
                        }
                    }
                    "go" => {
                        println!("bestmove {}", self.engine.search(1000_u64));
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

    pub fn send_info(
        depth: Option<usize>,
        nodes: Option<u64>,
        time: Option<u64>,
        nps: Option<u64>,
        score_cp: Option<i32>,
        pv: Option<&str>
    ) {
        let mut output = String::from("info");

        if let Some(d) = depth {
            output.push_str(&format!(" depth {}", d));
        }

        if let Some(s) = score_cp {
            output.push_str(&format!(" score cp {}", s));
        }

        if let Some(n) = nodes {
            output.push_str(&format!(" nodes {}", n));
        }

        if let Some(n) = nps {
            output.push_str(&format!(" nps {}", n));
        }

        if let Some(t) = time {
            output.push_str(&format!(" time {}", t));
        }

        if let Some(p) = pv {
            output.push_str(&format!(" pv {}", p));
        }

        println!("{}", output);
    }
}