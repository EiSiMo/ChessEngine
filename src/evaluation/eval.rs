use crate::Engine;
use crate::evaluation::consts;

impl Engine {
                                // TODO use pointer
    pub fn evaluate(&mut self, board: chess::Board) -> i32 {
        let mut score: i32 = 0;
        let w_figures = board.color_combined(chess::Color::White).0;
        let b_figures = board.color_combined(chess::Color::Black).0;


        let w_pawns = board.pieces(chess::Piece::Pawn).0 & w_figures;
        let b_pawns = board.pieces(chess::Piece::Pawn).0 & b_figures;
        let w_knights = board.pieces(chess::Piece::Knight).0 & w_figures;
        let b_knights = board.pieces(chess::Piece::Knight).0 & b_figures;
        let w_bishops = board.pieces(chess::Piece::Bishop).0 & w_figures;
        let b_bishops = board.pieces(chess::Piece::Bishop).0 & b_figures;
        let w_rooks = board.pieces(chess::Piece::Rook).0 & w_figures;
        let b_rooks = board.pieces(chess::Piece::Rook).0 & b_figures;
        let w_queens = board.pieces(chess::Piece::Queen).0 & w_figures;
        let b_queens = board.pieces(chess::Piece::Queen).0 & b_figures;
        let w_king = board.pieces(chess::Piece::King).0 & w_figures;
        let b_king = board.pieces(chess::Piece::King).0 & b_figures;


        score += ((w_pawns & consts::W_PAWN_100).count_ones() * 100) as i32;
        score += ((w_pawns & consts::W_PAWN_150).count_ones() * 150) as i32;
        score += ((w_pawns & consts::W_PAWN_110).count_ones() * 110) as i32;
        score += ((w_pawns & consts::W_PAWN_120).count_ones() * 120) as i32;
        score += ((w_pawns & consts::W_PAWN_130).count_ones() * 130) as i32;
        score += ((w_pawns & consts::W_PAWN_105).count_ones() * 105) as i32;
        score += ((w_pawns & consts::W_PAWN_127).count_ones() * 127) as i32;
        score += ((w_pawns & consts::W_PAWN_125).count_ones() * 125) as i32;
        score += ((w_pawns & consts::W_PAWN_095).count_ones() * 095) as i32;
        score += ((w_pawns & consts::W_PAWN_090).count_ones() * 090) as i32;
        score += ((w_pawns & consts::W_PAWN_075).count_ones() * 075) as i32;
        score -= ((b_pawns & consts::B_PAWN_100).count_ones() * 100) as i32;
        score -= ((b_pawns & consts::B_PAWN_105).count_ones() * 105) as i32;
        score -= ((b_pawns & consts::B_PAWN_110).count_ones() * 110) as i32;
        score -= ((b_pawns & consts::B_PAWN_075).count_ones() * 075) as i32;
        score -= ((b_pawns & consts::B_PAWN_095).count_ones() * 095) as i32;
        score -= ((b_pawns & consts::B_PAWN_090).count_ones() * 090) as i32;
        score -= ((b_pawns & consts::B_PAWN_125).count_ones() * 125) as i32;
        score -= ((b_pawns & consts::B_PAWN_127).count_ones() * 127) as i32;
        score -= ((b_pawns & consts::B_PAWN_120).count_ones() * 120) as i32;
        score -= ((b_pawns & consts::B_PAWN_130).count_ones() * 130) as i32;
        score -= ((b_pawns & consts::B_PAWN_150).count_ones() * 150) as i32;
        score += ((w_knights & consts::W_KNIGHT_270).count_ones() * 270) as i32;
        score += ((w_knights & consts::W_KNIGHT_280).count_ones() * 280) as i32;
        score += ((w_knights & consts::W_KNIGHT_290).count_ones() * 290) as i32;
        score += ((w_knights & consts::W_KNIGHT_300).count_ones() * 300) as i32;
        score += ((w_knights & consts::W_KNIGHT_320).count_ones() * 320) as i32;
        score += ((w_knights & consts::W_KNIGHT_330).count_ones() * 330) as i32;
        score += ((w_knights & consts::W_KNIGHT_335).count_ones() * 335) as i32;
        score += ((w_knights & consts::W_KNIGHT_325).count_ones() * 325) as i32;
        score += ((w_knights & consts::W_KNIGHT_340).count_ones() * 340) as i32;
        score -= ((b_knights & consts::B_KNIGHT_270).count_ones() * 270) as i32;
        score -= ((b_knights & consts::B_KNIGHT_280).count_ones() * 280) as i32;
        score -= ((b_knights & consts::B_KNIGHT_300).count_ones() * 300) as i32;
        score -= ((b_knights & consts::B_KNIGHT_290).count_ones() * 290) as i32;
        score -= ((b_knights & consts::B_KNIGHT_320).count_ones() * 320) as i32;
        score -= ((b_knights & consts::B_KNIGHT_325).count_ones() * 325) as i32;
        score -= ((b_knights & consts::B_KNIGHT_330).count_ones() * 330) as i32;
        score -= ((b_knights & consts::B_KNIGHT_335).count_ones() * 335) as i32;
        score -= ((b_knights & consts::B_KNIGHT_340).count_ones() * 340) as i32;
        score += ((w_bishops & consts::W_BISHOP_310).count_ones() * 310) as i32;
        score += ((w_bishops & consts::W_BISHOP_320).count_ones() * 320) as i32;
        score += ((w_bishops & consts::W_BISHOP_330).count_ones() * 330) as i32;
        score += ((w_bishops & consts::W_BISHOP_335).count_ones() * 335) as i32;
        score += ((w_bishops & consts::W_BISHOP_340).count_ones() * 340) as i32;
        score += ((w_bishops & consts::W_BISHOP_290).count_ones() * 290) as i32;
        score -= ((b_bishops & consts::B_BISHOP_310).count_ones() * 310) as i32;
        score -= ((b_bishops & consts::B_BISHOP_320).count_ones() * 320) as i32;
        score -= ((b_bishops & consts::B_BISHOP_290).count_ones() * 290) as i32;
        score -= ((b_bishops & consts::B_BISHOP_335).count_ones() * 335) as i32;
        score -= ((b_bishops & consts::B_BISHOP_330).count_ones() * 330) as i32;
        score -= ((b_bishops & consts::B_BISHOP_340).count_ones() * 340) as i32;
        score += ((w_rooks & consts::W_ROOK_500).count_ones() * 500) as i32;
        score -= ((b_rooks & consts::B_ROOK_500).count_ones() * 500) as i32;
        score += ((w_queens & consts::W_QUEEN_900).count_ones() * 900) as i32;
        score -= ((b_queens & consts::B_QUEEN_900).count_ones() * 900) as i32;
        score += ((w_king & consts::W_KING_MID_19970).count_ones() * 19970) as i32;
        score += ((w_king & consts::W_KING_MID_19960).count_ones() * 19960) as i32;
        score += ((w_king & consts::W_KING_MID_19950).count_ones() * 19950) as i32;
        score += ((w_king & consts::W_KING_MID_19980).count_ones() * 19980) as i32;
        score += ((w_king & consts::W_KING_MID_19990).count_ones() * 19990) as i32;
        score += ((w_king & consts::W_KING_MID_20020).count_ones() * 20020) as i32;
        score += ((w_king & consts::W_KING_MID_20000).count_ones() * 20000) as i32;
        score += ((w_king & consts::W_KING_MID_20030).count_ones() * 20030) as i32;
        score += ((w_king & consts::W_KING_MID_20010).count_ones() * 20010) as i32;
        score -= ((b_king & consts::B_KING_MID_20020).count_ones() * 20020) as i32;
        score -= ((b_king & consts::B_KING_MID_20030).count_ones() * 20030) as i32;
        score -= ((b_king & consts::B_KING_MID_20010).count_ones() * 20010) as i32;
        score -= ((b_king & consts::B_KING_MID_20000).count_ones() * 20000) as i32;
        score -= ((b_king & consts::B_KING_MID_19990).count_ones() * 19990) as i32;
        score -= ((b_king & consts::B_KING_MID_19980).count_ones() * 19980) as i32;
        score -= ((b_king & consts::B_KING_MID_19970).count_ones() * 19970) as i32;
        score -= ((b_king & consts::B_KING_MID_19960).count_ones() * 19960) as i32;
        score -= ((b_king & consts::B_KING_MID_19950).count_ones() * 19950) as i32;
        score += ((w_king & consts::W_KING_END_19950).count_ones() * 19950) as i32;
        score += ((w_king & consts::W_KING_END_19960).count_ones() * 19960) as i32;
        score += ((w_king & consts::W_KING_END_19970).count_ones() * 19970) as i32;
        score += ((w_king & consts::W_KING_END_19980).count_ones() * 19980) as i32;
        score += ((w_king & consts::W_KING_END_19990).count_ones() * 19990) as i32;
        score += ((w_king & consts::W_KING_END_20000).count_ones() * 20000) as i32;
        score += ((w_king & consts::W_KING_END_20020).count_ones() * 20020) as i32;
        score += ((w_king & consts::W_KING_END_20030).count_ones() * 20030) as i32;
        score += ((w_king & consts::W_KING_END_20040).count_ones() * 20040) as i32;
        score -= ((b_king & consts::B_KING_END_19950).count_ones() * 19950) as i32;
        score -= ((b_king & consts::B_KING_END_19970).count_ones() * 19970) as i32;
        score -= ((b_king & consts::B_KING_END_20000).count_ones() * 20000) as i32;
        score -= ((b_king & consts::B_KING_END_19990).count_ones() * 19990) as i32;
        score -= ((b_king & consts::B_KING_END_20020).count_ones() * 20020) as i32;
        score -= ((b_king & consts::B_KING_END_20030).count_ones() * 20030) as i32;
        score -= ((b_king & consts::B_KING_END_20040).count_ones() * 20040) as i32;
        score -= ((b_king & consts::B_KING_END_19980).count_ones() * 19980) as i32;
        score -= ((b_king & consts::B_KING_END_19960).count_ones() * 19960) as i32;

        score
    }
}
