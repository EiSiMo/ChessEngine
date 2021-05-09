use crate::board::Board;

use std::fmt;


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub fn as_fen(&self) -> String {
        let mut result = String::new();

        // insert the positions of the pieces
        let figures = self.get_figures();
        for line_index in 0_u8..8_u8 {
            let continuous_empty = 0_u8;
            let current_not_empty = true;

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