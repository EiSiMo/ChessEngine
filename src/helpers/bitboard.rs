pub fn count_pieces(bitboard: u64) -> f32 {
    let mut score: f32 = 0_f32;
    for shift in 0_u8..64_u8 {
        score += (bitboard >> shift & 1) as f32;
    }
    score
}