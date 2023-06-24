use cozy_chess::{Board, Color, Piece};

pub trait ScoreFunction {
    fn score(&self, board: Board) -> f64;
}

#[derive(Clone, Default)]
pub struct PawnDifferenceScore;

impl ScoreFunction for PawnDifferenceScore {
    fn score(&self, board: Board) -> f64 {
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
        Piece::Bishop => 3.0,
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
    fn test_pawn_difference_score() -> Result<(), FenParseError> {
        let score_fn = PawnDifferenceScore::default();

        let board = Board::default();
        assert_eq!(score_fn.score(board), 0.0);

        let board = Board::from_fen("rnbqkbnr/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", false)?;
        assert_eq!(score_fn.score(board), 8.0);

        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w KQkq - 0 1", false)?;
        assert_eq!(score_fn.score(board), -8.0);

        Ok(())
    }
}
