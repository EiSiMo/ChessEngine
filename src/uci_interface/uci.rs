use std::fs::OpenOptions;
use std::io::{self, Write};
use chess::{Board};


use crate::Engine;

impl Engine {
    pub fn uci_loop(&mut self) {
        while !self.quit {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.receive(input.trim());
        }
    }

    fn handle_position(&mut self, mut arg: &[&str]){
        if arg[0] == "startpos" {
            let mut board = Board::default();
            let mut result = Board::default();
            if arg.len() > 1 {
                arg = &arg[2..];
                for move_str in arg.iter() {

                    let square1 = chess::Square::from_string(move_str[0..2].to_string()).unwrap();
                    let square2 = chess::Square::from_string(move_str[2..4].to_string()).unwrap();

                    let mut move_ = chess::ChessMove::new(square1, square2, None);

                    // promotion
                    if move_str.len() == 5 {
                        let promotion_piece_char = move_str.chars().nth(4).unwrap();
                        let mut promotion_piece = chess::Piece::Queen;
                        if promotion_piece_char == 'p' {
                            promotion_piece = chess::Piece::Pawn;
                        } else if promotion_piece_char == 'r' {
                            promotion_piece = chess::Piece::Rook;
                        } else if promotion_piece_char == 'b' {
                            promotion_piece = chess::Piece::Bishop;
                        } else if promotion_piece_char == 'k' {
                            promotion_piece = chess::Piece::Knight;
                        }
                        move_ = chess::ChessMove::new(square1, square2, Some(promotion_piece));
                    }

                    board.make_move(move_, &mut result);
                    board = result;
                }
            }
            self.position = board;
        }
        // FEN try if matches ^(?:(?:[PNBRQK]+|[1-8])\/){7}(?:[PNBRQK]+|[1-8])$
        else if true {
        }
        // handle invalid input
        else {
        }
    }

    fn handle_uci_newgame(&mut self) {
        assert!(true);
    }

    fn handle_go(&mut self, _arg: &[&str]) {
        let depth = 4;
        let (may_best_move, _) = self.minmax(self.position, depth);
        match may_best_move {
            None => {
                println!("no move found");
            },
            Some(best_move) => {
                self.send(&format!("bestmove {}", best_move));
            }
        }
    }

    fn handle_uci(&self, _arg: &[&str]) {
        self.send(&format!("id name {}", self.name));
        self.send(&format!("id author {}", self.author));
        self.send("uciok");
    }
    fn handle_quit(&mut self) {
        self.quit = true;
    }

    fn handle_stop(&self) {
        assert!(true);
    }

    fn handle_ready(&self) {
        self.send("readyok");
    }

    // To handle commands that are not implemented by default in trait
    fn handle_unknown(&self, _command: &str, _arg: &[&str]){
        assert!(true);
    }

    fn print_board(&self){
        assert!(true);
    }

    fn is_keyword(&self, arg: &str) -> bool {
        matches!(
            arg,
            "position" | "go" | "uci" | "quit" | "stop" | "ponderhit" | "debug" | "isready"
        )
    }

    fn is_position(&self, arg: &str) -> bool {
        matches!(arg, "startpos" | "fen")
    }

    fn is_go(&self, arg: &str) -> bool {
        matches!(
            arg,
            "searchmoves"
                | "ponder"
                | "wtime"
                | "btime"
                | "winc"
                | "binc"
                | "movestogo"
                | "depth"
                | "nodes"
                | "mate"
                | "movetime"
                | "infinite"
        )
    }

    fn receive(&mut self, arg: &str) {
        self.log_stdio(&format!("Received: {}", arg));
        let mut args = arg.split_whitespace().collect::<Vec<&str>>();
        if args.is_empty() {
            return;
        }
        let command = args.remove(0);
        match command {
            "position" => self.handle_position(&args),
            "go" => self.handle_go(&args),
            "uci" => self.handle_uci(&args),
            "quit" => self.handle_quit(),
            "stop" => self.handle_stop(),
            "isready" => self.handle_ready(),
            "ucinewgame" => self.handle_uci_newgame(),
            "d" => self.print_board(),
            _ => self.handle_unknown(command, &args),
        }
    }

    fn send(&self, arg: &str) {
        self.log_stdio(&format!("Sent: {}", arg));
        println!("{}", arg);
    }

    fn send_ready_ok(&self) {
        self.send("readyok");
    }

    fn send_bestmove(&self, bestmove: &str) {
        self.send(format!("bestmove {}", bestmove).as_str());
    }

    fn send_info(&self, info: &str) {
        self.send(format!("info {}", info).as_str());
    }

    fn log_stdio(&self, content: &str) -> io::Result<()> {
        let file_path = String::from("C:\\Users\\morit\\Documents\\GitHub\\ChessEngine\\uci_log.txt");
        let mut file = OpenOptions::new().create(true).append(true).open(&file_path)?;
        writeln!(file, "{}", content)?;
        Ok(())
    }
}
