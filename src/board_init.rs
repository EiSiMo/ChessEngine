use crate::board::Board;

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