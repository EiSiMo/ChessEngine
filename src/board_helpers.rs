use crate::board::Board;

impl Board {
    pub fn get_enemies(self) -> u64 {
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

    pub fn get_allies(self) -> u64 {
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

    pub fn get_figures(self) -> u64 {
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

    pub fn get_blacks(self) -> u64 {
        self.black_pawns
            | self.black_bishops
            | self.black_knights
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    pub fn get_withes(self) -> u64 {
        self.withe_pawns
            | self.withe_bishops
            | self.withe_knights
            | self.withe_rooks
            | self.withe_queens
            | self.withe_king
    }

    pub fn delete_enemies(&mut self, squares: u64) {
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

    pub fn is_lost(&self) -> bool {
        self.withe_king == 0 || self.black_king == 0
    }

    pub fn next(&mut self) {
        self.next_turns_number += 1;
        self.withes_turn = !self.withes_turn;
    }
}