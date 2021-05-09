#[macro_use]
use lazy_static::lazy_static;

use crate::board::Board;

lazy_static! {
    static ref KNIGHT_MOVE_TABLE: [Vec<u64>; 64] = [
        vec!(9007199254740992, 70368744177664),
        vec!(4503599627370496, 35184372088832),
        vec!(36028797018963968, 2251799813685248, 70368744177664, 17592186044416),
        vec!(18014398509481984, 1125899906842624, 35184372088832, 8796093022208),
        vec!(9007199254740992, 562949953421312, 17592186044416, 4398046511104),
        vec!(4503599627370496, 281474976710656, 8796093022208, 2199023255552),
        vec!(2251799813685248, 4398046511104, 1099511627776),
        vec!(1125899906842624, 2199023255552),
        vec!(2305843009213693952, 35184372088832, 274877906944),
        vec!(1152921504606846976, 17592186044416, 137438953472),
        vec!(9223372036854775808, 576460752303423488, 140737488355328, 8796093022208, 274877906944, 68719476736),
        vec!(4611686018427387904, 288230376151711744, 70368744177664, 4398046511104, 137438953472, 34359738368),
        vec!(2305843009213693952, 144115188075855872, 35184372088832, 2199023255552, 68719476736, 17179869184),
        vec!(1152921504606846976, 72057594037927936, 17592186044416, 1099511627776, 34359738368, 8589934592),
        vec!(576460752303423488, 8796093022208, 17179869184, 4294967296),
        vec!(288230376151711744, 4398046511104, 8589934592),
        vec!(4611686018427387904, 9007199254740992, 137438953472, 1073741824),
        vec!(9223372036854775808, 2305843009213693952, 4503599627370496, 68719476736, 536870912),
        vec!(4611686018427387904, 1152921504606846976, 36028797018963968, 2251799813685248, 549755813888, 34359738368, 1073741824, 268435456),
        vec!(2305843009213693952, 576460752303423488, 18014398509481984, 1125899906842624, 274877906944, 17179869184, 536870912, 134217728),
        vec!(1152921504606846976, 288230376151711744, 9007199254740992, 562949953421312, 137438953472, 8589934592, 268435456, 67108864),
        vec!(576460752303423488, 144115188075855872, 4503599627370496, 281474976710656, 68719476736, 4294967296, 134217728, 33554432),
        vec!(288230376151711744, 72057594037927936, 2251799813685248, 34359738368, 67108864, 16777216),
        vec!(144115188075855872, 1125899906842624, 17179869184, 33554432),
        vec!(18014398509481984, 35184372088832, 536870912, 4194304),
        vec!(36028797018963968, 9007199254740992, 17592186044416, 268435456, 2097152),
        vec!(18014398509481984, 4503599627370496, 140737488355328, 8796093022208, 2147483648, 134217728, 4194304, 1048576),
        vec!(9007199254740992, 2251799813685248, 70368744177664, 4398046511104, 1073741824, 67108864, 2097152, 524288),
        vec!(4503599627370496, 1125899906842624, 35184372088832, 2199023255552, 536870912, 33554432, 1048576, 262144),
        vec!(2251799813685248, 562949953421312, 17592186044416, 1099511627776, 268435456, 16777216, 524288, 131072),
        vec!(1125899906842624, 281474976710656, 8796093022208, 134217728, 262144, 65536),
        vec!(562949953421312, 4398046511104, 67108864, 131072),
        vec!(70368744177664, 137438953472, 2097152, 16384),
        vec!(140737488355328, 35184372088832, 68719476736, 1048576, 8192),
        vec!(70368744177664, 17592186044416, 549755813888, 34359738368, 8388608, 524288, 16384, 4096),
        vec!(35184372088832, 8796093022208, 274877906944, 17179869184, 4194304, 262144, 8192, 2048),
        vec!(17592186044416, 4398046511104, 137438953472, 8589934592, 2097152, 131072, 4096, 1024),
        vec!(8796093022208, 2199023255552, 68719476736, 4294967296, 1048576, 65536, 2048, 512),
        vec!(4398046511104, 1099511627776, 34359738368, 524288, 1024, 256),
        vec!(2199023255552, 17179869184, 262144, 512),
        vec!(274877906944, 536870912, 8192, 64),
        vec!(549755813888, 137438953472, 268435456, 4096, 32),
        vec!(274877906944, 68719476736, 2147483648, 134217728, 32768, 2048, 64, 16),
        vec!(137438953472, 34359738368, 1073741824, 67108864, 16384, 1024, 32, 8),
        vec!(68719476736, 17179869184, 536870912, 33554432, 8192, 512, 16, 4),
        vec!(34359738368, 8589934592, 268435456, 16777216, 4096, 256, 8, 2),
        vec!(17179869184, 4294967296, 134217728, 2048, 4, 1),
        vec!(8589934592, 67108864, 1024, 2),
        vec!(1073741824, 2097152, 32),
        vec!(2147483648, 536870912, 1048576, 16),
        vec!(1073741824, 268435456, 8388608, 524288, 128, 8, 0),
        vec!(536870912, 134217728, 4194304, 262144, 64, 4, 0),
        vec!(268435456, 67108864, 2097152, 131072, 32, 2, 0),
        vec!(134217728, 33554432, 1048576, 65536, 16, 1, 0),
        vec!(67108864, 16777216, 524288, 8, 0),
        vec!(33554432, 262144, 4, 0),
        vec!(4194304, 8192),
        vec!(8388608, 2097152, 4096),
        vec!(4194304, 1048576, 32768, 2048),
        vec!(2097152, 524288, 16384, 1024),
        vec!(1048576, 262144, 8192, 512),
        vec!(524288, 131072, 4096, 256),
        vec!(262144, 65536, 2048),
        vec!(131072, 1024),
    ];
}


impl Board {
    pub fn generate_possible_moves(self) -> Vec<Board> {
        let mut moves: Vec<Board> = Vec::new();
        let figures = self.get_figures();
        let enemies = self.get_enemies();
        let allies = self.get_allies();

        if self.withe_king | self.black_king != 0 {
            if self.withes_turn {
                for field_index in 0_u8..64_u8 {
                    let first_field = 0b1000000000000000000000000000000000000000000000000000000000000000_u64;
                    let field = first_field >> field_index;

                    let pawn = self.withe_pawns & field;
                    let bishop = self.withe_bishops & field;
                    let knight = self.withe_knights & field;
                    let rook = self.withe_rooks & field;
                    let queen = self.withe_queens & field;
                    let king = self.withe_king & field;

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
                    else if bishop != 0 {
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
                    else if knight != 0 {
                        for square_jump in KNIGHT_MOVE_TABLE[field_index as usize].iter() {
                            let knight_blocked = allies & square_jump;
                            if knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.withe_knights ^= knight;
                                // insert the new piece
                                board_move.withe_knights |= square_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(*square_jump);
                                board_move.next();
                                moves.push(board_move);
                            }
                        }
                    }
                    else if rook != 0 {
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
                    else if queen != 0 {
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
                    else if king != 0 {
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
            } else {
                for field_index in 0_u8..64_u8 {
                    let first_field = 0b1000000000000000000000000000000000000000000000000000000000000000_u64;
                    let field: u64 = first_field >> field_index;

                    let pawn = self.black_pawns & field;
                    let bishop = self.black_bishops & field;
                    let knight = self.black_knights & field;
                    let rook = self.black_rooks & field;
                    let queen = self.black_queens & field;
                    let king = self.black_king & field;

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
                    else if bishop != 0 {
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
                    else if knight != 0 {
                        for square_jump in KNIGHT_MOVE_TABLE[field_index as usize].iter() {
                            let knight_blocked = allies & square_jump;
                            if knight_blocked == 0 {
                                // create a copy of the current board
                                let mut board_move = self.clone();
                                // delete the old piece
                                board_move.black_knights ^= knight;
                                // insert the new piece
                                board_move.black_knights |= square_jump;
                                // delete the enemy piece if existing
                                board_move.delete_enemies(*square_jump);
                                board_move.next();
                                moves.push(board_move);
                            }
                        }
                    }
                    else if rook != 0 {
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
                    else if queen != 0 {
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
                    else if king != 0 {
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
        moves
    }
}