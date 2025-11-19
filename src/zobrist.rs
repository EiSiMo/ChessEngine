use crate::board::{Color, PieceType};
use crate::square::Square;
use std::sync::OnceLock;

// We use a simple Xorshift generator to avoid external dependencies like 'rand'
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
    pub pieces: [[u64; 64]; 12], // [PieceType 0-5 + Color offset][Square]
    pub castling: [u64; 16],     // 16 combinations of castling rights
    pub en_passant: [u64; 9],    // 8 files + 1 for "no ep"
    pub side_to_move: u64,
}

// Thread-safe, write-once global storage
static KEYS: OnceLock<ZobristKeys> = OnceLock::new();

pub fn init_zobrist() {
    // If already initialized, do nothing
    if KEYS.get().is_some() {
        return;
    }

    let mut rng = Xorshift::new(1070372); // Fixed seed for reproducibility

    let mut pieces = [[0; 64]; 12];
    for i in 0..12 {
        for j in 0..64 {
            pieces[i][j] = rng.next();
        }
    }

    let mut castling = [0; 16];
    for i in 0..16 {
        castling[i] = rng.next();
    }

    let mut en_passant = [0; 9];
    for i in 0..9 {
        en_passant[i] = rng.next();
    }

    let side_to_move = rng.next();

    let keys = ZobristKeys {
        pieces,
        castling,
        en_passant,
        side_to_move,
    };

    // Set the global keys. Unwrap panics if set is called twice (should not happen).
    KEYS.set(keys).expect("Zobrist keys already initialized");
}

// Safe accessor without unsafe block
pub fn zobrist_keys() -> &'static ZobristKeys {
    KEYS.get().expect("Zobrist keys not initialized! Call init_zobrist() in main.")
}

// Helper to map piece+color to index 0-11
pub fn piece_index(pt: PieceType, c: Color) -> usize {
    let offset = match c {
        Color::White => 0,
        Color::Black => 6,
    };
    (pt as usize) + offset
}