use cozy_chess::{Board, Color, Piece};

pub trait ScoreFunction {
    // always returns score from white's perspective
    fn score(&self, board: &Board) -> f64;
}

#[derive(Clone, Default)]
pub struct PawnDifferenceScore;

impl ScoreFunction for PawnDifferenceScore {
    fn score(&self, board: &Board) -> f64 {
        let mut score = 0.0;

        for white_piece in board.colors(Color::White) {
            score += piece_value(board.piece_on(white_piece).expect("should be piece here"));
        }

        for black_piece in board.colors(Color::Black) {
            score -= piece_value(board.piece_on(black_piece).expect("should be piece here"));
        }

        score
    }
}

fn piece_value(piece: Piece) -> f64 {
    match piece {
        Piece::Pawn => 1.0,
        Piece::Knight => 3.0,
        Piece::Bishop => 3.5,
        Piece::Rook => 5.0,
        Piece::Queen => 9.0,
        Piece::King => 0.0,
    }
}

#[cfg(test)]
mod test {
    use cozy_chess::FenParseError;

    use super::*;

    #[test]
    fn basic_test_pawn_difference_score() -> Result<(), FenParseError> {
        let score_fn = PawnDifferenceScore::default();

        let board = Board::default();
        assert_eq!(score_fn.score(&board), 0.0);

        let board = Board::from_fen("rnbqkbnr/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", false)?;
        assert_eq!(score_fn.score(&board), 8.0);

        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w KQkq - 0 1", false)?;
        assert_eq!(score_fn.score(&board), -8.0);

        Ok(())
    }

    #[test]
    fn game_test_pawn_difference_score() -> Result<(), FenParseError> {
        let score_fn = PawnDifferenceScore::default();

        // new game: score should be 0
        let mut board = Board::startpos();
        assert_eq!(score_fn.score(&board), 0.0);

        // white pawn from D2 to D4: score should be 0
        board.play("d2d4".parse().unwrap());
        assert_eq!(score_fn.score(&board), 0.0);

        // black pawn from E7 to E5: score should be 0
        board.play("e7e5".parse().unwrap());
        assert_eq!(score_fn.score(&board), 0.0);

        // white pawn takes on E5: score should be 1.0
        board.play("d4e5".parse().unwrap());
        assert_eq!(score_fn.score(&board), 1.0);

        // black moves queen to G5: score should be 1.0
        board.play("d8g5".parse().unwrap());
        assert_eq!(score_fn.score(&board), 1.0);

        // white pushes pawn to E6: score should be 1.0
        board.play("e5e6".parse().unwrap());
        assert_eq!(score_fn.score(&board), 1.0);

        // black queen takes bishop on C1: score should be -2.5
        board.play("g5c1".parse().unwrap());
        assert_eq!(score_fn.score(&board), -2.5);

        Ok(())
    }
}
