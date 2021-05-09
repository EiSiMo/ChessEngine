#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub withe_pawns: u64,
    pub withe_bishops: u64,
    pub withe_knights: u64,
    pub withe_rooks: u64,
    pub withe_queens: u64,
    pub withe_king: u64,
    pub black_pawns: u64,
    pub black_bishops: u64,
    pub black_knights: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,

    // false when it's blacks turn
    pub withes_turn: bool,
    pub next_turns_number: u16,
}
