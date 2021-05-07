use std::fmt;

use super::super::enums;
use super::super::helpers::bitboard::count_pieces;
use std::fmt::Formatter;
use std::ops::Deref;
use std::time::{Duration, SystemTime};
use rand::seq::SliceRandom;
use std::cmp;
use std::cmp::max;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    withe_pawns: u64,
    withe_bishops: u64,
    withe_knights: u64,
    withe_rooks: u64,
    withe_queens: u64,
    withe_king: u64,
    black_pawns: u64,
    black_bishops: u64,
    black_knights: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,

    // false when it's blacks turn
    withes_turn: bool,
    next_turns_number: u16,
}

// board representation
impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        // add top border
        result.push_str("---------------------\n");

        // add top coordinates line
        result.push_str("   A B C D E F G H\n");

        for line_index in 0_u8..8_u8 {
            // add left coordinates line
            result.push_str((line_index + 1).to_string().as_str());
            result.push_str("  ");

            for col_index in 0_u8..8_u8 {
                let first_field = 0b1000000000000000000000000000000000000000000000000000000000000000_u64;
                let mask = first_field >> (line_index * 8 + col_index);
                if self.withe_pawns & mask != 0 {
                    result.push_str("P")
                } else if self.withe_bishops & mask != 0 {
                    result.push_str("B")
                } else if self.withe_knights & mask != 0 {
                    result.push_str("N")
                } else if self.withe_rooks & mask != 0 {
                    result.push_str("R")
                } else if self.withe_queens & mask != 0 {
                    result.push_str("Q")
                } else if self.withe_king & mask != 0 {
                    result.push_str("K")
                } else if self.black_pawns & mask != 0 {
                    result.push_str("p")
                } else if self.black_bishops & mask != 0 {
                    result.push_str("b")
                } else if self.black_knights & mask != 0 {
                    result.push_str("n")
                } else if self.black_rooks & mask != 0 {
                    result.push_str("r")
                } else if self.black_queens & mask != 0 {
                    result.push_str("q")
                } else if self.black_king & mask != 0 {
                    result.push_str("k")
                } else {
                    result.push_str(" ")
                }
                result.push_str(" ")
            }

            // add right coordinates line
            result.push_str(" ");
            result.push_str((line_index + 1).to_string().as_str());
            result.push_str("\n");
        }

        // add bot coordinates line
        result.push_str("   A B C D E F G H");

        // add bot border
        result.push_str("\n---------------------");


        write!(f, "{}", result)
    }
}

impl Board {
    pub fn as_fen(&mut self) -> String {
        let mut result = String::new();

        // insert the positions of the pieces
        let figures = self.get_figures();
        for line_index in 0_u8..8_u8 {
            let mut continuous_empty = 0_u8;
            let mut current_not_empty = true;

            for col_index in 0_u8..8_u8 {
                let first_field = 0b1000000000000000000000000000000000000000000000000000000000000000_u64;
                let mask = first_field >> (line_index * 8 + col_index);
                if figures & mask == 0 {
                    if result.len() == 0 {
                        result.push_str("0");
                    }
                    // None if the last char is no digit
                    let last_digit = result.chars().last().unwrap().to_digit(10);
                    match last_digit {
                        None => {
                            result.push_str("1");
                        }
                        Some(mut last_digit) => {
                            last_digit += 1;
                            result.pop();
                            result.push_str(last_digit.to_string().as_str());
                        }
                    }
                } else if self.withe_pawns & mask != 0 {
                    result.push_str("P")
                } else if self.withe_bishops & mask != 0 {
                    result.push_str("B")
                } else if self.withe_knights & mask != 0 {
                    result.push_str("N")
                } else if self.withe_rooks & mask != 0 {
                    result.push_str("R")
                } else if self.withe_queens & mask != 0 {
                    result.push_str("Q")
                } else if self.withe_king & mask != 0 {
                    result.push_str("K")
                } else if self.black_pawns & mask != 0 {
                    result.push_str("p")
                } else if self.black_bishops & mask != 0 {
                    result.push_str("b")
                } else if self.black_knights & mask != 0 {
                    result.push_str("n")
                } else if self.black_rooks & mask != 0 {
                    result.push_str("r")
                } else if self.black_queens & mask != 0 {
                    result.push_str("q")
                } else if self.black_king & mask != 0 {
                    result.push_str("k")
                }
                if current_not_empty == true && continuous_empty > 0 {
                    result.push_str(continuous_empty.to_string().as_str());
                }
            }
            result.push_str("/")
        }
        result.pop();

        // insert how has to move
        if self.withes_turn {
            result.push_str(" w")
        } else {
            result.push_str(" b");
        }

        // insert castling rights
        // TODO: not implemented
        result.push_str(" -");

        // insert en passant rights
        // TODO: not implemented
        result.push_str(" -");

        // insert halfmove number
        // TODO: not implemented
        result.push_str(" 0");

        // insert fullmove number
        result.push_str(" ");
        result.push_str(self.next_turns_number.to_string().as_str());

        result
    }
}

// board creation
impl Board {
    pub fn empty() -> Board {
        Board {
            withe_pawns:   0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            withe_bishops: 0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            withe_knights: 0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            withe_rooks:   0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            withe_queens:  0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            withe_king:    0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            black_pawns:   0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            black_bishops: 0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            black_knights: 0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            black_rooks:   0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            black_queens:  0b0000000000000000000000000000000000000000000000000000000000000000_u64,
            black_king:    0b0000000000000000000000000000000000000000000000000000000000000000_u64,

            // false if its blacks turn
            withes_turn: true,
            next_turns_number: 1,
        }
    }

    pub fn default() -> Board {
        Board {
            withe_pawns:   0b0000000000000000000000000000000000000000000000001111111100000000_u64,
            withe_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100_u64,
            withe_knights: 0b0000000000000000000000000000000000000000000000000000000001000010_u64,
            withe_rooks:   0b0000000000000000000000000000000000000000000000000000000010000001_u64,
            withe_queens:  0b0000000000000000000000000000000000000000000000000000000000010000_u64,
            withe_king:    0b0000000000000000000000000000000000000000000000000000000000001000_u64,
            black_pawns:   0b0000000011111111000000000000000000000000000000000000000000000000_u64,
            black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000_u64,
            black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000_u64,
            black_rooks:   0b1000000100000000000000000000000000000000000000000000000000000000_u64,
            black_queens:  0b0001000000000000000000000000000000000000000000000000000000000000_u64,
            black_king:    0b0000100000000000000000000000000000000000000000000000000000000000_u64,

            withes_turn: true,
            next_turns_number: 1,
        }
    }

    pub fn from_fen(fen: &str) -> Board {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        // extract the position of the pieces
        let positions: Vec<char> = parts[0].replace("/", "").chars().collect();
        let mut board = Board::empty();
        let mut mask = 0b1000000000000000000000000000000000000000000000000000000000000000_u64;
        for character in positions {
            match character {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    let shift: u32 = character.to_digit(10).unwrap();
                    mask >>= (shift - 1);
                }
                'P' => board.withe_pawns |= mask,
                'B' => board.withe_bishops |= mask,
                'N' => board.withe_knights |= mask,
                'R' => board.withe_rooks |= mask,
                'Q' => board.withe_queens |= mask,
                'K' => board.withe_king |= mask,
                'p' => board.black_pawns |= mask,
                'b' => board.black_bishops |= mask,
                'n' => board.black_knights |= mask,
                'r' => board.black_rooks |= mask,
                'q' => board.black_queens |= mask,
                'k' => board.black_king |= mask,
                _ => {
                    println!("Invalid char: {}", character);
                    break;
                }
            }
            mask >>= 1;
        }

        // extract who has to move
        if parts[1] == "b" {
            // default is true
            board.withes_turn = false;
        }

        // extract th number of the next turn
        board.next_turns_number = parts[5].parse().unwrap();

        board
    }
}

// move finding
impl Board {
    pub fn move_random(&mut self) -> Board {
        let possible_moves = self.generate_possible_moves();
        let random_move = possible_moves.choose(&mut rand::thread_rng());
        *random_move.unwrap()
    }
/*
    pub fn move_best(&mut self, depth: u8) -> Board {
        let mut possible_moves = self.generate_possible_moves();

        let mut best_score = possible_moves[0].evaluate();
        let mut best_move = possible_moves[0];

        if self.withes_turn {
            for mut possible_move in possible_moves {
                let score = possible_move.minimax(
                    depth - 1,
                    f32::MIN,
                    f32::MAX);
                if score > best_score {
                    best_score = score;
                    best_move = possible_move;
                }
            }
        } else {
            for mut possible_move in possible_moves {
                let score = possible_move.minimax(
                    depth - 1,
                    f32::MIN,
                    f32::MAX);
                if score < best_score {
                    best_score = score;
                    best_move = possible_move;
                }
            }
        }
        best_move
    }
*/
    pub fn minimax(&mut self, depth: u8, mut alpha: f32, mut beta: f32) -> (Board, f32) {
        if depth == 0 {
            let score = self.evaluate();
            return (*self, score)
        }

        let mut possible_moves = self.generate_possible_moves();

        let mut best_score = possible_moves[0].evaluate();
        let mut best_move = possible_moves[0];

        if self.withes_turn {
            for mut possible_move in possible_moves {
                let result = possible_move.minimax(depth - 1, alpha, beta);
                let eval = result.1;
                if eval > best_score {
                    best_score = eval;
                    best_move = possible_move;
                }
                if eval > alpha {
                    alpha = eval
                }
                if beta <= alpha {
                    break;
                }
            }
            (best_move, best_score)
        } else {
            for mut possible_move in possible_moves {
                let result = possible_move.minimax(depth - 1, alpha, beta);
                let eval = result.1;
                if eval < best_score {
                    best_score = eval;
                    best_move = possible_move;
                }
                if eval < beta {
                    beta = eval
                }
                if beta <= alpha {
                    break;
                }
            }
            (best_move, best_score)
        }
    }

    pub fn evaluate(&mut self) -> f32 {
        /// positive: withe is better
        /// negative: black is better

        let mut score: f32 = 0_f32;
        for shift in 0_u8..64_u8 {
            score += (self.withe_pawns >> shift & 1) as f32;
            score += (self.withe_bishops >> shift & 1 * 3) as f32;
            score += (self.withe_knights >> shift & 1 * 3) as f32;
            score += (self.withe_rooks >> shift & 1 * 5) as f32;
            score += (self.withe_queens >> shift & 1 * 9) as f32;
            score += (self.black_king >> shift & 1 * 1024) as f32;
            score -= (self.black_pawns >> shift & 1) as f32;
            score -= (self.black_bishops >> shift & 1 * 3) as f32;
            score -= (self.black_knights >> shift & 1 * 3) as f32;
            score -= (self.black_rooks >> shift & 1 * 5) as f32;
            score -= (self.black_queens >> shift & 1 * 9) as f32;
            score -= (self.black_king >> shift & 1 * 1024) as f32;
        }
        score
    }
}

// move generation
impl Board {
    pub fn generate_possible_moves(&mut self) -> Vec<Board> {
        let mut moves: Vec<Board> = Vec::new();
        let figures = self.get_figures();
        let enemies = self.get_enemies();
        let allies = self.get_allies();

        if self.withe_king | self.black_king != 0 {
            if self.withes_turn {
                for field_index in 0_u8..64_u8 {
                    let first_field = 0b1000000000000000000000000000000000000000000000000000000000000000_u64;
                    let field = first_field >> field_index;
                    // WITHE PAWNS
                    {
                        let pawn = self.withe_pawns & field;
                        if pawn != 0 {
                            // ONE STEP
                            // exclude pawns on last two ranks
                            let mask_last = 0b1111111111111111000000000000000000000000000000000000000000000000_u64;
                            let pawn_last_rank = mask_last & pawn;
                            // get the square to step on
                            let pawn_one_step = pawn << 8;
                            // check if the square to step on is blocked
                            let pawn_one_step_blocked = figures & pawn_one_step;
                            // if the move is valid
                            if pawn_one_step_blocked == 0 && pawn_last_rank == 0 {
                                // PROMOTION
                                // exclude pawns not in the pre last rank
                                let mask_pre_last = 0b0000000011111111000000000000000000000000000000000000000000000000_u64;
                                let pawn_pre_last = mask_pre_last & pawn;
                                // if the move is valid
                                if pawn_pre_last != 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_knights |= pawn_one_step;
                                    board_move.next();
                                    moves.push(board_move);

                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_queens |= pawn_one_step;
                                    board_move.next();
                                    moves.push(board_move);
                                } else {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_pawns |= pawn_one_step;
                                    board_move.next();
                                    moves.push(board_move);
                                }
                            }

                            // CAPTURE
                            // exclude pawn in the right border or last rank
                            let mask_right = 0b1111111100000001000000010000000100000001000000010000000100000001_u64;
                            let pawn_not_right = !mask_right & pawn;
                            // exclude pawns with no enemy on the right diagonal square
                            let pawn_right_diagonal = pawn_not_right << 7;
                            let pawn_right_diagonal_enemy = pawn_right_diagonal & enemies;

                            // exclude pawns in the left border or last rank
                            let mask_left = 0b1111111110000000100000001000000010000000100000001000000010000000_u64;
                            let pawn_not_left = !mask_left & pawn;
                            // exclude pawns with no enemy on the left diagonal square
                            let pawn_left_diagonal = pawn_not_left << 9;
                            let pawn_left_diagonal_enemy = pawn_left_diagonal & enemies;

                            // if the move is valid
                            if pawn_right_diagonal_enemy != 0 {
                                // diagonal promotion
                                let mask_pre_last = 0b0000000011111111000000000000000000000000000000000000000000000000_u64;
                                let pawn_pre_last = mask_pre_last & pawn;
                                if pawn_pre_last != 0 {
                                    // diagonal promotion capture
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_queens |= pawn_right_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_right_diagonal);
                                    board_move.next();
                                    moves.push(board_move);

                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_knights |= pawn_right_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_right_diagonal);
                                    board_move.next();
                                    moves.push(board_move);
                                } else {
                                    // diagonal capture
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_pawns |= pawn_right_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_right_diagonal);
                                    board_move.next();
                                    moves.push(board_move);
                                }
                            }
                            if pawn_left_diagonal_enemy != 0 {
                                let mask_pre_last = 0b0000000011111111000000000000000000000000000000000000000000000000_u64;
                                let pawn_pre_last = mask_pre_last & pawn;
                                if pawn_pre_last != 0 {
                                    // diagonal promotion capture
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_queens |= pawn_left_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_left_diagonal);
                                    board_move.next();
                                    moves.push(board_move);

                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_knights |= pawn_left_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_left_diagonal);
                                    board_move.next();
                                    moves.push(board_move);
                                } else {
                                    // diagonal capture
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.withe_pawns |= pawn_left_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_left_diagonal);
                                    board_move.next();
                                    moves.push(board_move);
                                }
                            }

                            // TWO STEP
                            // exclude pawns not on the starting rank
                            let mask_second_rank = 0b0000000000000000000000000000000000000000000000001111111100000000_u64;
                            let pawn_second_rank = pawn & mask_second_rank;

                            // get the square to jump over
                            let pawn_jump = pawn_second_rank << 8;
                            // check if the square to jump over is blocked
                            let pawn_jump_blocked = figures & pawn_jump;
                            // get the square to step on
                            let pawn_two_step = pawn_second_rank << 16;
                            // check if the square to step on is blocked
                            let pawn_two_step_blocked = figures & pawn_two_step;

                            // if the move is valid
                            if pawn_second_rank != 0 && pawn_two_step_blocked == 0 && pawn_jump_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_pawns ^= pawn;
                                // insert the new piece
                                board_move.withe_pawns |= pawn_two_step;
                                board_move.next();
                                moves.push(board_move);
                            }
                        }
                    }

                    // WITHE BISHOP
                    {
                        let bishop = self.withe_bishops & field;
                        if bishop != 0 {
                            // TOP LEFT DIAGONAL
                            let mut bishop_step = bishop << 9;
                            // contains the position of the piece before the move now added
                            let mut bishop_last_step = bishop;
                            loop {
                                // exclude bishops in 1 left column or 1 top line
                                let tl_mask = 0b1111111110000000100000001000000010000000100000001000000010000000_u64;
                                let bishop_mask = tl_mask & bishop_last_step;
                                // exclude field blocked by allies
                                let step_blocked = bishop_step & allies;
                                if bishop_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_bishops ^= bishop;
                                    // insert the new piece
                                    board_move.withe_bishops |= bishop_step;
                                    // if enemy piece exists delete it break the loop
                                    let enemy_exists = enemies & bishop_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(bishop_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    bishop_last_step = bishop_step;
                                    bishop_step <<= 9;
                                } else {
                                    break;
                                }
                            }

                            // TOP RIGHT DIAGONAL
                            // exclude bishops in 1 right column or 1 top line
                            let mut bishop_step = bishop << 7;
                            // contains the position of the piece before the move now added
                            let mut bishop_last_step = bishop;
                            loop {
                                // exclude bishops in 1 left column or 1 top line
                                let tr_mask = 0b1111111100000001000000010000000100000001000000010000000100000001_u64;
                                let bishop_mask = tr_mask & bishop_last_step;
                                // exclude field blocked by allies
                                let step_blocked = bishop_step & allies;
                                if bishop_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_bishops ^= bishop;
                                    // insert the new piece
                                    board_move.withe_bishops |= bishop_step;
                                    // if enemy piece exists delete it break the loop
                                    let enemy_exists = enemies & bishop_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(bishop_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    bishop_last_step = bishop_step;
                                    bishop_step <<= 7;
                                } else {
                                    break;
                                }
                            }

                            // BOT LEFT DIAGONAL
                            let mut bishop_step = bishop >> 7;
                            // contains the position of the piece before the move now added
                            let mut bishop_last_step = bishop;
                            loop {
                                // exclude bishops in 1 left column or 1 top line
                                let bl_mask = 0b1000000010000000100000001000000010000000100000001000000011111111_u64;
                                let bishop_mask = bl_mask & bishop_last_step;
                                // exclude field blocked by allies
                                let step_blocked = bishop_step & allies;
                                if bishop_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_bishops ^= bishop;
                                    // insert the new piece
                                    board_move.withe_bishops |= bishop_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & bishop_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(bishop_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    bishop_last_step = bishop_step;
                                    bishop_step >>= 7;
                                } else {
                                    break;
                                }
                            }

                            // BOT RIGHT DIAGONAL
                            let mut bishop_step = bishop >> 9;
                            // contains the position of the piece before the move now added
                            let mut bishop_last_step = bishop;
                            loop {
                                // exclude bishops in 1 right column or 1 bot line
                                let br_mask = 0b0000000100000001000000010000000100000001000000010000000111111111_u64;
                                let bishop_mask = br_mask & bishop_last_step;
                                // exclude field blocked by allies
                                let step_blocked = bishop_step & allies;
                                if bishop_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_bishops ^= bishop;
                                    // insert the new piece
                                    board_move.withe_bishops |= bishop_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & bishop_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(bishop_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    bishop_last_step = bishop_step;
                                    bishop_step >>= 9;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // WITHE KNIGHT
                    {
                        let knight = self.withe_knights & field;
                        if knight != 0 {
                            // TOP LEFT TOP JUMP
                            // exclude knights in 1 left column or 2 top lines
                            let tlt_mask = 0b1111111111111111100000001000000010000000100000001000000010000000_u64;
                            let knight_mask = tlt_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight << 17;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // TOP RIGHT TOP JUMP
                            // exclude knights in 1 right column or 2 top lines
                            let trt_mask = 0b1111111111111111000000010000000100000001000000010000000100000001_u64;
                            let knight_mask = trt_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight << 15;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // TOP LEFT BOT JUMP
                            // exclude knights in 2 left columns or 1 top line
                            let tlb_mask = 0b1111111111000000110000001100000011000000110000001100000011000000_u64;
                            let knight_mask = tlb_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight << 10;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // TOP RIGHT BOT JUMP
                            // exclude knights in 2 right columns or 1 top line
                            let trb_mask = 0b1111111100000011000000110000001100000011000000110000001100000011_u64;
                            let knight_mask = trb_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight << 6;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // BOT LEFT TOP JUMP
                            // exclude knights in 2 left columns or 1 bot line
                            let blt_mask = 0b1100000011000000110000001100000011000000110000001100000011111111_u64;
                            let knight_mask = blt_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight >> 6;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // BOT RIGHT TOP JUMP
                            // exclude knights in 2 right columns or 1 bot line
                            let brt_mask = 0b0000001100000011000000110000001100000011000000110000001111111111_u64;
                            let knight_mask = brt_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight >> 10;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // BOT LEFT BOT JUMP
                            // exclude knights in 1 left column or 2 bot lines
                            let blb_mask = 0b1000000010000000100000001000000010000000100000001111111111111111_u64;
                            let knight_mask = blb_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight >> 15;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // BOT RIGHT BOT JUMP
                            // exclude knights in 1 right column or 2 bot lines
                            let brb_mask = 0b0000000100000001000000010000000100000001000000011111111111111111_u64;
                            let knight_mask = brb_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight >> 17;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }
                        }
                    }

                    // WITHE ROOK
                    {
                        let rook = self.withe_rooks & field;
                        if rook != 0 {
                            // TOP
                            let mut rook_step = rook << 8;
                            // contains the position of the piece before the move now added
                            let mut rook_last_step = rook;
                            loop {
                                // exclude rooks in top line
                                let tl_mask = 0b1111111100000000000000000000000000000000000000000000000000000000_u64;
                                let rook_mask = tl_mask & rook_last_step;
                                // exclude field blocked by allies
                                let step_blocked = rook_step & allies;
                                if rook_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_rooks ^= rook;
                                    // insert the new piece
                                    board_move.withe_rooks |= rook_step;
                                    // if enemy piece existing delete it and break the loop
                                    let enemy_exists = enemies & rook_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(rook_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);

                                    rook_last_step = rook_step;
                                    rook_step <<= 8;
                                } else {
                                    break;
                                }
                            }

                            // BOT
                            // exclude rooks in 1 right column or 1 top line
                            let mut rook_step = rook >> 8;
                            // contains the position of the piece before the move now added
                            let mut rook_last_step = rook;
                            loop {
                                // exclude rooks in bot line
                                let tr_mask = 0b0000000000000000000000000000000000000000000000000000000011111111_u64;
                                let rook_mask = tr_mask & rook_last_step;
                                // exclude field blocked by allies
                                let step_blocked = rook_step & allies;
                                if rook_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_rooks ^= rook;
                                    // insert the new piece
                                    board_move.withe_rooks |= rook_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & rook_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(rook_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    rook_last_step = rook_step;
                                    rook_step >>= 8;
                                } else {
                                    break;
                                }
                            }

                            // LEFT
                            let mut rook_step = rook << 1;
                            // contains the position of the piece before the move now added
                            let mut rook_last_step = rook;
                            loop {
                                // exclude rooks in left column
                                let bl_mask = 0b1000000010000000100000001000000010000000100000001000000010000000_u64;
                                let rook_mask = bl_mask & rook_last_step;
                                // exclude field blocked by allies
                                let step_blocked = rook_step & allies;
                                if rook_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_rooks ^= rook;
                                    // insert the new piece
                                    board_move.withe_rooks |= rook_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & rook_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(rook_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    rook_last_step = rook_step;
                                    rook_step <<= 1;
                                } else {
                                    break;
                                }
                            }

                            // RIGHT
                            let mut rook_step = rook >> 1;
                            // contains the position of the piece before the move now added
                            let mut rook_last_step = rook;
                            loop {
                                // exclude rooks in right column
                                let br_mask = 0b0000000100000001000000010000000100000001000000010000000100000001_u64;
                                let rook_mask = br_mask & rook_last_step;
                                // exclude field blocked by allies
                                let step_blocked = rook_step & allies;
                                if rook_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_rooks ^= rook;
                                    // insert the new piece
                                    board_move.withe_rooks |= rook_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & rook_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(rook_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    rook_last_step = rook_step;
                                    rook_step >>= 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // WITHE QUEEN
                    {
                        let queen = self.withe_queens & field;
                        if queen != 0 {
                            // TOP LEFT DIAGONAL
                            let mut queen_step = queen << 9;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in 1 left column or 1 top line
                                let tl_mask = 0b1111111110000000100000001000000010000000100000001000000010000000_u64;
                                let queen_mask = tl_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_queens ^= queen;
                                    // insert the new piece
                                    board_move.withe_queens |= queen_step;
                                    // remove enemy piece if existing
                                    board_move.delete_enemies(queen_step);
                                    board_move.next();
                                    moves.push(board_move);

                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        break;
                                    }
                                    queen_last_step = queen_step;
                                    queen_step <<= 9;
                                } else {
                                    break;
                                }
                            }

                            // TOP RIGHT DIAGONAL
                            // exclude queens in 1 right column or 1 top line
                            let mut queen_step = queen << 7;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in 1 left column or 1 top line
                                let tr_mask = 0b1111111100000001000000010000000100000001000000010000000100000001_u64;
                                let queen_mask = tr_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_queens ^= queen;
                                    // insert the new piece
                                    board_move.withe_queens |= queen_step;
                                    // remove enemy piece if existing
                                    board_move.delete_enemies(queen_step);
                                    board_move.next();
                                    moves.push(board_move);

                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        break;
                                    }
                                    queen_last_step = queen_step;
                                    queen_step <<= 7;
                                } else {
                                    break;
                                }
                            }

                            // BOT LEFT DIAGONAL
                            let mut queen_step = queen >> 7;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in 1 left column or 1 top line
                                let bl_mask = 0b1000000010000000100000001000000010000000100000001000000011111111_u64;
                                let queen_mask = bl_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_queens ^= queen;
                                    // insert the new piece
                                    board_move.withe_queens |= queen_step;
                                    // remove enemy piece if existing
                                    board_move.delete_enemies(queen_step);
                                    board_move.next();
                                    moves.push(board_move);

                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        break;
                                    }
                                    queen_last_step = queen_step;
                                    queen_step >>= 7;
                                } else {
                                    break;
                                }
                            }

                            // BOT RIGHT DIAGONAL
                            let mut queen_step = queen >> 9;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in 1 right column or 1 bot line
                                let br_mask = 0b0000000100000001000000010000000100000001000000010000000111111111_u64;
                                let queen_mask = br_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_queens ^= queen;
                                    // insert the new piece
                                    board_move.withe_queens |= queen_step;
                                    // remove enemy piece if existing
                                    board_move.delete_enemies(queen_step);
                                    board_move.next();
                                    moves.push(board_move);

                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        break;
                                    }
                                    queen_last_step = queen_step;
                                    queen_step >>= 9;
                                } else {
                                    break;
                                }
                            }

                            // TOP
                            let mut queen_step = queen << 8;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in top line
                                let tl_mask = 0b1111111100000000000000000000000000000000000000000000000000000000_u64;
                                let queen_mask = tl_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_queens ^= queen;
                                    // insert the new piece
                                    board_move.withe_queens |= queen_step;
                                    // remove enemy piece if existing
                                    board_move.delete_enemies(queen_step);
                                    board_move.next();
                                    moves.push(board_move);

                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        break;
                                    }
                                    queen_last_step = queen_step;
                                    queen_step <<= 8;
                                } else {
                                    break;
                                }
                            }

                            // BOT
                            // exclude queens in 1 right column or 1 top line
                            let mut queen_step = queen >> 8;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in bot line
                                let tr_mask = 0b0000000000000000000000000000000000000000000000000000000011111111_u64;
                                let queen_mask = tr_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_queens ^= queen;
                                    // insert the new piece
                                    board_move.withe_queens |= queen_step;
                                    // remove enemy piece if existing
                                    board_move.delete_enemies(queen_step);
                                    board_move.next();
                                    moves.push(board_move);

                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        break;
                                    }
                                    queen_last_step = queen_step;
                                    queen_step >>= 8;
                                } else {
                                    break;
                                }
                            }

                            // LEFT
                            let mut queen_step = queen << 1;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in left column
                                let bl_mask = 0b1000000010000000100000001000000010000000100000001000000010000000_u64;
                                let queen_mask = bl_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_queens ^= queen;
                                    // insert the new piece
                                    board_move.withe_queens |= queen_step;
                                    // remove enemy piece if existing
                                    board_move.delete_enemies(queen_step);
                                    board_move.next();
                                    moves.push(board_move);

                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        break;
                                    }
                                    queen_last_step = queen_step;
                                    queen_step <<= 1;
                                } else {
                                    break;
                                }
                            }

                            // RIGHT
                            let mut queen_step = queen >> 1;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in right column
                                let br_mask = 0b0000000100000001000000010000000100000001000000010000000100000001_u64;
                                let queen_mask = br_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.withe_queens ^= queen;
                                    // insert the new piece
                                    board_move.withe_queens |= queen_step;
                                    // remove enemy piece if existing
                                    board_move.delete_enemies(queen_step);
                                    board_move.next();
                                    moves.push(board_move);

                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        break;
                                    }
                                    queen_last_step = queen_step;
                                    queen_step >>= 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // WITHE KING
                    {
                        let king = self.withe_king & field;
                        if king != 0 {
                            // TOP LEFT DIAGONAL
                            // exclude kings in the left column and top line
                            let mask = 0b1111111110000000100000001000000010000000100000001000000010000000_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king << 9;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_king ^= king;
                                // insert the new piece
                                board_move.withe_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // TOP
                            // exclude kings in the top line
                            let mask = 0b1111111100000000000000000000000000000000000000000000000000000000_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king << 8;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_king ^= king;
                                // insert the new piece
                                board_move.withe_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // TOP RIGHT DIAGONAL
                            // exclude kings in the right column and top line
                            let mask = 0b1111111100000001000000010000000100000001000000010000000100000001_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king << 7;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_king ^= king;
                                // insert the new piece
                                board_move.withe_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // BOT LEFT DIAGONAL
                            // exclude kings in the left column and bot line
                            let mask = 0b1000000010000000100000001000000010000000100000001000000011111111_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king >> 7;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_king ^= king;
                                // insert the new piece
                                board_move.withe_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // BOT
                            // exclude kings in the bot line
                            let mask = 0b0000000000000000000000000000000000000000000000000000000011111111_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king >> 8;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_king ^= king;
                                // insert the new piece
                                board_move.withe_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // BOT RIGHT DIAGONAL
                            // exclude kings in the right column and bot line
                            let mask = 0b0000000100000001000000010000000100000001000000010000000111111111_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king >> 9;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_king ^= king;
                                // insert the new piece
                                board_move.withe_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // LEFT
                            // exclude kings in the left column
                            let mask = 0b1000000010000000100000001000000010000000100000001000000010000000_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king << 1;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_king ^= king;
                                // insert the new piece
                                board_move.withe_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // RIGHT
                            // exclude kings in the right column
                            let mask = 0b0000000100000001000000010000000100000001000000010000000100000001_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king >> 1;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_king ^= king;
                                // insert the new piece
                                board_move.withe_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                        }
                    }
                }
            } else {
                for field_index in 0_u8..64_u8 {
                    let first_field = 0b1000000000000000000000000000000000000000000000000000000000000000_u64;
                    let field: u64 = first_field >> field_index;

                    // BLACK PAWN
                    {
                        let pawn = self.black_pawns & field;
                        if pawn != 0 {
                            // ONE STEP
                            // exclude pawns on last two ranks
                            let mask_last = 0b0000000000000000000000000000000000000000000000001111111111111111_u64;
                            let pawn_last_rank = mask_last & pawn;
                            // get the square to step on
                            let pawn_one_step = pawn >> 8;
                            // check if the square to step on is blocked
                            let pawn_one_step_blocked = figures & pawn_one_step;
                            // if the move is valid
                            if pawn_one_step_blocked == 0 && pawn_last_rank == 0 {
                                // PROMOTION
                                // exclude pawns not in the pre last rank
                                let mask_pre_last = 0b000000000000000000000000000000000000000000000000111111110000000_u64;
                                let pawn_pre_last = mask_pre_last & pawn;
                                // if the move is valid
                                if pawn_pre_last != 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_knights |= pawn_one_step;
                                    board_move.next();
                                    moves.push(board_move);

                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_queens |= pawn_one_step;
                                    board_move.next();
                                    moves.push(board_move);
                                } else {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_pawns |= pawn_one_step;
                                    board_move.next();
                                    moves.push(board_move);
                                }
                            }

                            // TWO STEP
                            // exclude pawns not on the starting rank
                            let mask_second_rank = 0b0000000011111111000000000000000000000000000000000000000000000000_u64;
                            let pawn_second_rank = pawn & mask_second_rank;

                            // get the square to jump over
                            let pawn_jump = pawn_second_rank >> 8;
                            // check if the square to jump over is blocked
                            let pawn_jump_blocked = figures & pawn_jump;
                            // get the square to step on
                            let pawn_two_step = pawn_second_rank >> 16;
                            // check if the square to step on is blocked
                            let pawn_two_step_blocked = figures & pawn_two_step;

                            // if the move is valid
                            if pawn_second_rank != 0 && pawn_two_step_blocked == 0 && pawn_jump_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_pawns ^= pawn;
                                // insert the new piece
                                board_move.black_pawns |= pawn_two_step;
                                board_move.next();
                                moves.push(board_move);
                            }

                            // CAPTURE
                            // exclude pawn in the right border or last rank
                            let mask_right = 0b0000000100000001000000010000000100000001000000010000000111111111_u64;
                            let pawn_not_right = !mask_right & pawn;
                            // exclude pawns with no enemy on the right diagonal square
                            let pawn_right_diagonal = pawn_not_right >> 9;
                            let pawn_right_diagonal_enemy = pawn_right_diagonal & enemies;

                            // exclude pawns in the left border or last rank
                            let mask_left = 0b1000000010000000100000001000000010000000100000001000000011111111_u64;
                            let pawn_not_left = !mask_left & pawn;
                            // exclude pawns with no enemy on the left diagonal square
                            let pawn_left_diagonal = pawn_not_left >> 7;
                            let pawn_left_diagonal_enemy = pawn_left_diagonal & enemies;

                            if pawn_right_diagonal_enemy != 0 {
                                // diagonal promotion
                                let mask_pre_last = 0b000000000000000000000000000000000000000000000000111111110000000_u64;
                                let pawn_pre_last = mask_pre_last & pawn;
                                if pawn_pre_last != 0 {
                                    // diagonal promotion capture
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_queens |= pawn_right_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_right_diagonal);
                                    board_move.next();
                                    moves.push(board_move);

                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_knights |= pawn_right_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_right_diagonal);
                                    board_move.next();
                                    moves.push(board_move);
                                } else {
                                    // diagonal capture
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_pawns |= pawn_right_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_right_diagonal);
                                    board_move.next();
                                    moves.push(board_move);
                                }
                            }
                            if pawn_left_diagonal_enemy != 0 {
                                let mask_pre_last = 0b000000000000000000000000000000000000000000000000111111110000000_u64;
                                let pawn_pre_last = mask_pre_last & pawn;
                                if pawn_pre_last != 0 {
                                    // diagonal promotion capture
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_queens |= pawn_left_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_left_diagonal);
                                    board_move.next();
                                    moves.push(board_move);

                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_knights |= pawn_left_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_left_diagonal);
                                    board_move.next();
                                    moves.push(board_move);
                                } else {
                                    // diagonal capture
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_pawns ^= pawn;
                                    // insert the new piece
                                    board_move.black_pawns |= pawn_left_diagonal;
                                    // delete the enemy piece
                                    board_move.delete_enemies(pawn_left_diagonal);
                                    board_move.next();
                                    moves.push(board_move);
                                }
                            }
                        }
                    }

                    // BLACK BISHOP
                    {
                        let bishop = self.black_bishops & field;
                        if bishop != 0 {
                            // TOP LEFT DIAGONAL
                            let mut bishop_step = bishop << 9;
                            // contains the position of the piece before the move now added
                            let mut bishop_last_step = bishop;
                            loop {
                                // exclude bishops in 1 left column or 1 top line
                                let tl_mask = 0b1111111110000000100000001000000010000000100000001000000010000000_u64;
                                let bishop_mask = tl_mask & bishop_last_step;
                                // exclude field blocked by allies
                                let step_blocked = bishop_step & allies;
                                if bishop_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_bishops ^= bishop;
                                    // insert the new piece
                                    board_move.black_bishops |= bishop_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & bishop_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(bishop_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    bishop_last_step = bishop_step;
                                    bishop_step <<= 9;
                                } else {
                                    break;
                                }
                            }

                            // TOP RIGHT DIAGONAL
                            // exclude bishops in 1 right column or 1 top line
                            let mut bishop_step = bishop << 7;
                            // contains the position of the piece before the move now added
                            let mut bishop_last_step = bishop;
                            loop {
                                // exclude bishops in 1 left column or 1 top line
                                let tr_mask = 0b1111111100000001000000010000000100000001000000010000000100000001_u64;
                                let bishop_mask = tr_mask & bishop_last_step;
                                // exclude field blocked by allies
                                let step_blocked = bishop_step & allies;
                                if bishop_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_bishops ^= bishop;
                                    // insert the new piece
                                    board_move.black_bishops |= bishop_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & bishop_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(bishop_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    bishop_last_step = bishop_step;
                                    bishop_step <<= 7;
                                } else {
                                    break;
                                }
                            }

                            // BOT LEFT DIAGONAL
                            let mut bishop_step = bishop >> 7;
                            // contains the position of the piece before the move now added
                            let mut bishop_last_step = bishop;
                            loop {
                                // exclude bishops in 1 left column or 1 top line
                                let bl_mask = 0b1000000010000000100000001000000010000000100000001000000011111111_u64;
                                let bishop_mask = bl_mask & bishop_last_step;
                                // exclude field blocked by allies
                                let step_blocked = bishop_step & allies;
                                if bishop_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_bishops ^= bishop;
                                    // insert the new piece
                                    board_move.black_bishops |= bishop_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & bishop_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(bishop_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    bishop_last_step = bishop_step;
                                    bishop_step >>= 7;
                                } else {
                                    break;
                                }
                            }

                            // BOT RIGHT DIAGONAL
                            let mut bishop_step = bishop >> 9;
                            // contains the position of the piece before the move now added
                            let mut bishop_last_step = bishop;
                            loop {
                                // exclude bishops in 1 right column or 1 bot line
                                let br_mask = 0b0000000100000001000000010000000100000001000000010000000111111111_u64;
                                let bishop_mask = br_mask & bishop_last_step;
                                // exclude field blocked by allies
                                let step_blocked = bishop_step & allies;
                                if bishop_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_bishops ^= bishop;
                                    // insert the new piece
                                    board_move.black_bishops |= bishop_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & bishop_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(bishop_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    bishop_last_step = bishop_step;
                                    bishop_step >>= 9;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // BLACK KNIGHT
                    {
                        let knight = self.black_knights & field;
                        if knight != 0 {
                            // TOP LEFT TOP JUMP
                            // exclude knights in 1 left column or 2 top lines
                            let tlt_mask = 0b1111111111111111100000001000000010000000100000001000000010000000_u64;
                            let knight_mask = tlt_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight << 17;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // TOP RIGHT TOP JUMP
                            // exclude knights in 1 right column or 2 top lines
                            let trt_mask = 0b1111111111111111000000010000000100000001000000010000000100000001_u64;
                            let knight_mask = trt_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight << 15;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // TOP LEFT BOT JUMP
                            // exclude knights in 2 left columns or 1 top line
                            let tlb_mask = 0b1111111111000000110000001100000011000000110000001100000011000000_u64;
                            let knight_mask = tlb_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight << 10;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // TOP RIGHT BOT JUMP
                            // exclude knights in 2 right columns or 1 top line
                            let trb_mask = 0b1111111100000011000000110000001100000011000000110000001100000011_u64;
                            let knight_mask = trb_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight << 6;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // BOT LEFT TOP JUMP
                            // exclude knights in 2 left columns or 1 bot line
                            let blt_mask = 0b1100000011000000110000001100000011000000110000001100000011111111_u64;
                            let knight_mask = blt_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight >> 6;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // BOT RIGHT TOP JUMP
                            // exclude knights in 2 right columns or 1 bot line
                            let brt_mask = 0b0000001100000011000000110000001100000011000000110000001111111111_u64;
                            let knight_mask = brt_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight >> 10;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // BOT LEFT BOT JUMP
                            // exclude knights in 1 left column or 2 bot lines
                            let blb_mask = 0b1000000010000000100000001000000010000000100000001111111111111111_u64;
                            let knight_mask = blb_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight >> 15;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }

                            // BOT RIGHT BOT JUMP
                            // exclude knights in 1 right column or 2 bot lines
                            let brb_mask = 0b0000000100000001000000010000000100000001000000011111111111111111_u64;
                            let knight_mask = brb_mask & knight;
                            // exclude fields blocked by allies
                            let knight_jump = knight >> 17;
                            let knight_blocked = allies & knight_jump;
                            // if the move is valid
                            if knight_mask == 0 && knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= knight_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(knight_jump);
                                board_move.next();
                                moves.push(board_move);
                            }
                        }
                    }

                    // BLACK ROOK
                    {
                        let rook = self.black_rooks & field;
                        if rook != 0 {
                            // TOP
                            let mut rook_step = rook << 8;
                            // contains the position of the piece before the move now added
                            let mut rook_last_step = rook;
                            loop {
                                // exclude rooks in 1 left column or 1 top line
                                let tl_mask = 0b1111111100000000000000000000000000000000000000000000000000000000_u64;
                                let rook_mask = tl_mask & rook_last_step;
                                // exclude field blocked by allies
                                let step_blocked = rook_step & allies;
                                if rook_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_rooks ^= rook;
                                    // insert the new piece
                                    board_move.black_rooks |= rook_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & rook_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(rook_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    rook_last_step = rook_step;
                                    rook_step <<= 8;
                                } else {
                                    break;
                                }
                            }

                            // BOT
                            // exclude rooks in 1 right column or 1 top line
                            let mut rook_step = rook >> 8;
                            // contains the position of the piece before the move now added
                            let mut rook_last_step = rook;
                            loop {
                                // exclude rooks in 1 left column or 1 top line
                                let tr_mask = 0b0000000000000000000000000000000000000000000000000000000011111111_u64;
                                let rook_mask = tr_mask & rook_last_step;
                                // exclude field blocked by allies
                                let step_blocked = rook_step & allies;
                                if rook_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_rooks ^= rook;
                                    // insert the new piece
                                    board_move.black_rooks |= rook_step;
                                    // if enemy piece exists delete and break the loop
                                    let enemy_exists = enemies & rook_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(rook_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    rook_last_step = rook_step;
                                    rook_step >>= 8;
                                } else {
                                    break;
                                }
                            }

                            // LEFT
                            let mut rook_step = rook << 1;
                            // contains the position of the piece before the move now added
                            let mut rook_last_step = rook;
                            loop {
                                // exclude rooks in 1 left column or 1 top line
                                let bl_mask = 0b1000000010000000100000001000000010000000100000001000000010000000_u64;
                                let rook_mask = bl_mask & rook_last_step;
                                // exclude field blocked by allies
                                let step_blocked = rook_step & allies;
                                if rook_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_rooks ^= rook;
                                    // insert the new piece
                                    board_move.black_rooks |= rook_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & rook_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(rook_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    rook_last_step = rook_step;
                                    rook_step <<= 1;
                                } else {
                                    break;
                                }
                            }

                            // RIGHT
                            let mut rook_step = rook >> 1;
                            // contains the position of the piece before the move now added
                            let mut rook_last_step = rook;
                            loop {
                                // exclude rooks in 1 right column or 1 bot line
                                let br_mask = 0b0000000100000001000000010000000100000001000000010000000100000001_u64;
                                let rook_mask = br_mask & rook_last_step;
                                // exclude field blocked by allies
                                let step_blocked = rook_step & allies;
                                if rook_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_rooks ^= rook;
                                    // insert the new piece
                                    board_move.black_rooks |= rook_step;
                                    // if enemy piece exists delete it break the loop
                                    let enemy_exists = enemies & rook_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(rook_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    rook_last_step = rook_step;
                                    rook_step >>= 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // BLACK QUEEN
                    {
                        let queen = self.black_queens & field;
                        if queen != 0 {
                            // TOP LEFT DIAGONAL
                            let mut queen_step = queen << 9;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in 1 left column or 1 top line
                                let tl_mask = 0b1111111110000000100000001000000010000000100000001000000010000000_u64;
                                let queen_mask = tl_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_queens ^= queen;
                                    // insert the new piece
                                    board_move.black_queens |= queen_step;
                                    // if enemy piece existed, break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(queen_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    queen_last_step = queen_step;
                                    queen_step <<= 9;
                                } else {
                                    break;
                                }
                            }

                            // TOP RIGHT DIAGONAL
                            // exclude queens in 1 right column or 1 top line
                            let mut queen_step = queen << 7;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in 1 left column or 1 top line
                                let tr_mask = 0b1111111100000001000000010000000100000001000000010000000100000001_u64;
                                let queen_mask = tr_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_queens ^= queen;
                                    // insert the new piece
                                    board_move.black_queens |= queen_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(queen_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    queen_last_step = queen_step;
                                    queen_step <<= 7;
                                } else {
                                    break;
                                }
                            }

                            // BOT LEFT DIAGONAL
                            let mut queen_step = queen >> 7;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in 1 left column or 1 top line
                                let bl_mask = 0b1000000010000000100000001000000010000000100000001000000011111111_u64;
                                let queen_mask = bl_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_queens ^= queen;
                                    // insert the new piece
                                    board_move.black_queens |= queen_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(queen_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    queen_last_step = queen_step;
                                    queen_step >>= 7;
                                } else {
                                    break;
                                }
                            }

                            // BOT RIGHT DIAGONAL
                            let mut queen_step = queen >> 9;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in 1 right column or 1 bot line
                                let br_mask = 0b0000000100000001000000010000000100000001000000010000000111111111_u64;
                                let queen_mask = br_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_queens ^= queen;
                                    // insert the new piece
                                    board_move.black_queens |= queen_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(queen_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    queen_last_step = queen_step;
                                    queen_step >>= 9;
                                } else {
                                    break;
                                }
                            }

                            // TOP
                            let mut queen_step = queen << 8;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in top line
                                let tl_mask = 0b1111111100000000000000000000000000000000000000000000000000000000_u64;
                                let queen_mask = tl_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_queens ^= queen;
                                    // insert the new piece
                                    board_move.black_queens |= queen_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(queen_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    queen_last_step = queen_step;
                                    queen_step <<= 8;
                                } else {
                                    break;
                                }
                            }

                            // BOT
                            // exclude queens in 1 right column or 1 top line
                            let mut queen_step = queen >> 8;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in bot line
                                let tr_mask = 0b0000000000000000000000000000000000000000000000000000000011111111_u64;
                                let queen_mask = tr_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_queens ^= queen;
                                    // insert the new piece
                                    board_move.black_queens |= queen_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(queen_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    queen_last_step = queen_step;
                                    queen_step >>= 8;
                                } else {
                                    break;
                                }
                            }

                            // LEFT
                            let mut queen_step = queen << 1;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in left column
                                let bl_mask = 0b1000000010000000100000001000000010000000100000001000000010000000_u64;
                                let queen_mask = bl_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_queens ^= queen;
                                    // insert the new piece
                                    board_move.black_queens |= queen_step;
                                    // if enemy piece exists delete it and break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(queen_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    queen_last_step = queen_step;
                                    queen_step <<= 1;
                                } else {
                                    break;
                                }
                            }

                            // RIGHT
                            let mut queen_step = queen >> 1;
                            // contains the position of the piece before the move now added
                            let mut queen_last_step = queen;
                            loop {
                                // exclude queens in right column
                                let br_mask = 0b0000000100000001000000010000000100000001000000010000000100000001_u64;
                                let queen_mask = br_mask & queen_last_step;
                                // exclude field blocked by allies
                                let step_blocked = queen_step & allies;
                                if queen_mask == 0 && step_blocked == 0 {
                                    // create a copy of the current board
                                    let mut board_move = self.clone();
                                    // delete the old piece
                                    board_move.black_queens ^= queen;
                                    // insert the new piece
                                    board_move.black_queens |= queen_step;
                                    // if enemy piece exists delete it break the loop
                                    let enemy_exists = enemies & queen_step;
                                    if enemy_exists != 0 {
                                        board_move.delete_enemies(queen_step);
                                        break;
                                    }
                                    board_move.next();
                                    moves.push(board_move);
                                    queen_last_step = queen_step;
                                    queen_step >>= 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // BLACK KING
                    {
                        let king = self.black_king & field;
                        if king != 0 {
                            // TOP LEFT DIAGONAL
                            // exclude kings in the left column and top line
                            let mask = 0b1111111110000000100000001000000010000000100000001000000010000000_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king << 9;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_king ^= king;
                                // insert the new piece
                                board_move.black_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // TOP
                            // exclude kings in the top line
                            let mask = 0b1111111100000000000000000000000000000000000000000000000000000000_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king << 8;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_king ^= king;
                                // insert the new piece
                                board_move.black_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // TOP RIGHT DIAGONAL
                            // exclude kings in the right column and top line
                            let mask = 0b1111111100000001000000010000000100000001000000010000000100000001_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king << 7;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_king ^= king;
                                // insert the new piece
                                board_move.black_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // BOT LEFT DIAGONAL
                            // exclude kings in the left column and bot line
                            let mask = 0b1000000010000000100000001000000010000000100000001000000011111111_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king >> 7;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_king ^= king;
                                // insert the new piece
                                board_move.black_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // BOT
                            // exclude kings in the bot line
                            let mask = 0b0000000000000000000000000000000000000000000000000000000011111111_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king >> 8;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_king ^= king;
                                // insert the new piece
                                board_move.black_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // BOT RIGHT DIAGONAL
                            // exclude kings in the right column and bot line
                            let mask = 0b0000000100000001000000010000000100000001000000010000000111111111_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king >> 9;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_king ^= king;
                                // insert the new piece
                                board_move.black_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // LEFT
                            // exclude kings in the left column
                            let mask = 0b1000000010000000100000001000000010000000100000001000000010000000_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king << 1;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_king ^= king;
                                // insert the new piece
                                board_move.black_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                            // RIGHT
                            // exclude kings in the right column
                            let mask = 0b0000000100000001000000010000000100000001000000010000000100000001_u64;
                            let king_mask = mask & king;
                            // exclude field blocked by allies
                            let king_step = king >> 1;
                            let step_blocked = king_step & allies;
                            if king_mask == 0 && step_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_king ^= king;
                                // insert the new piece
                                board_move.black_king |= king_step;
                                // delete the enemy piece
                                board_move.delete_enemies(king_step);
                                board_move.next();
                                moves.push(board_move);
                            }
                        }
                    }
                }
            }
        }
        moves
    }
}

// helpers
impl Board {
    fn get_enemies(&mut self) -> u64 {
        if self.withes_turn {
            self.black_pawns
                | self.black_bishops
                | self.black_knights
                | self.black_rooks
                | self.black_queens
                | self.black_king
        } else {
            self.withe_pawns
                | self.withe_bishops
                | self.withe_knights
                | self.withe_rooks
                | self.withe_queens
                | self.withe_king
        }
    }

    fn get_allies(&mut self) -> u64 {
        if self.withes_turn {
            self.withe_pawns
                | self.withe_bishops
                | self.withe_knights
                | self.withe_rooks
                | self.withe_queens
                | self.withe_king
        } else {
            self.black_pawns
                | self.black_bishops
                | self.black_knights
                | self.black_rooks
                | self.black_queens
                | self.black_king
        }
    }

    fn get_figures(&mut self) -> u64 {
        self.withe_pawns
            | self.withe_bishops
            | self.withe_knights
            | self.withe_rooks
            | self.withe_queens
            | self.withe_king
            | self.black_pawns
            | self.black_bishops
            | self.black_knights
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    fn delete_enemies(&mut self, squares: u64) {
        if self.withes_turn {
            self.black_pawns &= !squares;
            self.black_bishops &= !squares;
            self.black_knights &= !squares;
            self.black_rooks &= !squares;
            self.black_queens &= !squares;
            self.black_king &= !squares;
        } else {
            self.withe_pawns &= !squares;
            self.withe_bishops &= !squares;
            self.withe_knights &= !squares;
            self.withe_rooks &= !squares;
            self.withe_queens &= !squares;
            self.withe_king &= !squares;
        }
    }

    fn next(&mut self) {
        self.next_turns_number += 1;
        self.withes_turn = !self.withes_turn;
    }
}
