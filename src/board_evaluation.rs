use crate::board::Board;

// credit https://www.chessprogramming.org/Simplified_Evaluation_Function
const PAWN_MULTIPLIER: i32 = 100;
const BISHOP_MULTIPLIER: i32 = 330;
const KNIGHT_MULTIPLIER: i32 = 320;
const ROOK_MULTIPLIER: i32 = 500;
const QUEEN_MULTIPLIER: i32 = 900;
const KING_MULTIPLIER: i32 = 20000;

// credit https://adamberent.com/2019/03/02/piece-square-table/
const WITHE_PAWN_TABLE: [i32; 64] =
[
     0,   0,   0,   0,   0,   0,   0,   0,
    50,  50,  50,  50,  50,  50,  50,  50,
    10,  10,  20,  30,  30,  20,  10,  10,
     5,   5,  10,  27,  27,  10,   5,   5,
     0,   0,   0,  25,  25,   0,   0,   0,
     5,  -5, -10,   0,   0, -10,  -5,   5,
     5,  10,  10, -25, -25,  10,  10,   5,
     0,   0,   0,   0,   0,   0,   0,   0
];

const BLACK_PAWN_TABLE: [i32; 64] =
[
     0,   0,   0,   0,   0,   0,   0,   0,
     5,  10,  10, -25, -25,  10,  10,   5,
     5,  -5, -10,   0,   0, -10,  -5,   5,
     0,   0,   0,  25,  25,   0,   0,   0,
     5,   5,  10,  27,  27,  10,   5,   5,
    10,  10,  20,  30,  30,  20,  10,  10,
    50,  50,  50,  50,  50,  50,  50,  50,
     0,   0,   0,   0,   0,   0,   0,   0
];

const WITHE_KNIGHT_TABLE: [i32; 64] =
[
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20 ,  0,   0,   0 ,  0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -20, -30, -30, -20, -40, -50,
];

const BLACK_KNIGHT_TABLE: [i32; 64] =
[
    -50, -40, -20, -30, -30, -20, -40, -50,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50
];

const WITHE_BISHOP_TABLE: [i32; 64] =
[
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -40, -10, -10, -40, -10, -20,
];

const BLACK_BISHOP_TABLE: [i32; 64] =
[
    -20, -10, -40, -10, -10, -40, -10, -20,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -20, -10, -10, -10, -10, -10, -10, -20
];

const WITHE_ROOK_TABLE: [i32; 64] =
[
     0,   0,   0,   0,   0,   0,   0,   0,
     5,  10,  10,  10,  10,  10,  10,   5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
     0,   0,   0,   5,   5,   0,   0,   0
];

const BLACK_ROOK_TABLE: [i32; 64] =
[
     0,   0,   0,   5,   5,   0,   0,   0,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
     5,  10,  10,  10,  10,  10,  10,   5,
     0,   0,   0,   0,   0,   0,   0,   0
];

const WITHE_QUEEN_TABLE: [i32; 64] =
[
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,   5,   5,   5,   0, -10,
     -5,   0,   5,   5,   5,   5,   0,  -5,
      0,   0,   5,   5,   5,   5,   0,  -5,
    -10,   5,   5,   5,   5,   5,   0, -10,
    -10,   0,   5,   0,   0,   0,   0, -10,
    -20, -10, -10,  -5,  -5, -10, -10, -20
];

const BLACK_QUEEN_TABLE: [i32; 64] =
[
      0,   0,   0,   0,   0,   0,   0,   0,
      5,  10,  10,  10,  10,  10,  10,   5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
      0,   0,   0,   5,   5,   0,   0,   0
];


impl Board {
    pub fn evaluate(&mut self) -> i32 {
        let withe_pawn_amount = self.withe_pawns.count_ones() as i32;
        let withe_bishop_amount = self.withe_bishops.count_ones() as i32;
        let withe_knight_amount = self.withe_knights.count_ones() as i32;
        let withe_rook_amount = self.withe_rooks.count_ones() as i32;
        let withe_queen_amount = self.withe_queens.count_ones() as i32;
        let withe_king_amount = self.withe_king.count_ones() as i32;

        let black_pawn_amount = self.black_pawns.count_ones() as i32;
        let black_bishop_amount = self.black_bishops.count_ones() as i32;
        let black_knight_amount = self.black_knights.count_ones() as i32;
        let black_rook_amount = self.black_rooks.count_ones() as i32;
        let black_queen_amount = self.black_queens.count_ones() as i32;
        let black_king_amount = self.black_king.count_ones() as i32;

        let mut score = 0_i32;
        score += withe_pawn_amount * PAWN_MULTIPLIER;
        score += withe_bishop_amount * BISHOP_MULTIPLIER;
        score += withe_knight_amount * KNIGHT_MULTIPLIER;
        score += withe_rook_amount * ROOK_MULTIPLIER;
        score += withe_queen_amount * QUEEN_MULTIPLIER;
        score += withe_king_amount * KING_MULTIPLIER;
        score -= black_pawn_amount * PAWN_MULTIPLIER;
        score -= black_bishop_amount * BISHOP_MULTIPLIER;
        score -= black_knight_amount * KNIGHT_MULTIPLIER;
        score -= black_rook_amount * ROOK_MULTIPLIER;
        score -= black_queen_amount * QUEEN_MULTIPLIER;
        score -= black_king_amount * KING_MULTIPLIER;


        for index in 0..64 {
            score += (self.withe_pawns >> index & 1) as i32 * WITHE_PAWN_TABLE[index];
            score += (self.withe_knights >> index & 1) as i32 * WITHE_KNIGHT_TABLE[index];
            score += (self.withe_bishops >> index & 1) as i32 * WITHE_BISHOP_TABLE[index];
            score += (self.withe_rooks >> index & 1) as i32 * WITHE_ROOK_TABLE[index];
            score += (self.withe_queens >> index & 1) as i32 * WITHE_QUEEN_TABLE[index];
            score -= (self.black_pawns >> index & 1) as i32 * BLACK_PAWN_TABLE[index];
            score -= (self.black_knights >> index & 1) as i32 * BLACK_KNIGHT_TABLE[index];
            score -= (self.black_bishops >> index & 1) as i32 * BLACK_BISHOP_TABLE[index];
            score -= (self.black_rooks >> index & 1) as i32 * WITHE_ROOK_TABLE[index];
            score -= (self.black_queens >> index & 1) as i32 * WITHE_QUEEN_TABLE[index];
        }

        score
}
}
