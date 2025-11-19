use crate::board::{Color, PieceType};
use std::sync::OnceLock;

struct Xorshift {
    state: u64,
}

impl Xorshift {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

#[derive(Debug)]
pub struct ZobristKeys {
    pub pieces: [[u64; 64]; 12],
    pub castling: [u64; 16],
    pub en_passant: [u64; 9],
    pub side_to_move: u64,
}

static KEYS: OnceLock<ZobristKeys> = OnceLock::new();

pub fn init_zobrist() {
    if KEYS.get().is_some() {
        return;
    }

    let mut rng = Xorshift::new(1070372); // Fixed seed for reproducibility

    let mut pieces = [[0; 64]; 12];
    for piece_squares in pieces.iter_mut() {
        for square_key in piece_squares.iter_mut() {
            *square_key = rng.next();
        }
    }

    let mut castling = [0; 16];
    for c in castling.iter_mut() {
        *c = rng.next();
    }

    let mut en_passant = [0; 9];
    for ep in en_passant.iter_mut() {
        *ep = rng.next();
    }

    let side_to_move = rng.next();

    let keys = ZobristKeys {
        pieces,
        castling,
        en_passant,
        side_to_move,
    };

    KEYS.set(keys).expect("Zobrist keys already initialized");
}

pub fn zobrist_keys() -> &'static ZobristKeys {
    KEYS.get().expect("Zobrist keys not initialized! Call init_zobrist() in main.")
}

pub fn piece_index(pt: PieceType, c: Color) -> usize {
    let offset = match c {
        Color::White => 0,
        Color::Black => 6,
    };
    (pt as usize) + offset
}