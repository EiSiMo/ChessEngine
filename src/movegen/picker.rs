use crate::board::Board;
use crate::movegen::non_sliders::{generate_king_moves, generate_knight_moves};
use crate::movegen::pawns::generate_pawn_moves;
use crate::movegen::sliders::*;
use crate::r#move::{Move, MoveList};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum GenStage {
    Pawns = 1,
    Knights = 2,
    Bishops = 3,
    Rooks = 4,
    Queens = 5,
    King = 6,
    Done = 7,
}

impl GenStage {
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Pawns => Some(Self::Knights),
            Self::Knights => Some(Self::Bishops),
            Self::Bishops => Some(Self::Rooks),
            Self::Rooks => Some(Self::Queens),
            Self::Queens => Some(Self::King),
            Self::King => Some(Self::Done),
            Self::Done => None,
        }
    }
}

pub struct MoveGenerator {
    buffer: MoveList,
    stage: GenStage,
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            buffer: MoveList::new(),
            stage: GenStage::Pawns,
        }
    }

    fn generate_next_batch(&mut self, board: &Board) {
        self.buffer.clear();

        match self.stage {
            GenStage::Pawns => { generate_pawn_moves(board, &mut self.buffer) }
            GenStage::Knights => { generate_knight_moves(board, &mut self.buffer) }
            GenStage::Bishops => { generate_bishop_moves(board, &mut self.buffer) }
            GenStage::Rooks => { generate_rook_moves(board, &mut self.buffer) }
            GenStage::Queens => { generate_queen_moves(board, &mut self.buffer) }
            GenStage::King => { generate_king_moves(board, &mut self.buffer) }
            GenStage::Done => {}
        }
        if let Some(next_stage) = self.stage.next() {
            self.stage = next_stage;
        }
    }

    pub fn next(&mut self, board: &Board) -> Option<Move> {
        loop {
            if let Some(mv) = self.buffer.pull() { return Some(mv) }
            if self.stage == GenStage::Done { return None }
            self.generate_next_batch(board);
        }
    }
}