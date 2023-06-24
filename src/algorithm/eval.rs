use super::score::ScoreFunction;
use cozy_chess::{Board, Color, Move};

pub trait Evaluator {
    fn eval_moves<T: ScoreFunction>(&self, board: Board, score_fn: &T) -> Vec<(Move, f64)>;
}

#[derive(Default, Clone)]
pub struct NaiveEvaluator;

impl Evaluator for NaiveEvaluator {
    fn eval_moves<T: ScoreFunction>(&self, board: Board, score_fn: &T) -> Vec<(Move, f64)> {
        let side = board.side_to_move();
        let moves = get_sorted_moves(&board, &side);

        moves
            .into_iter()
            .map(|mov| {
                let mut temp_board = board.clone();
                temp_board.play(mov);
                (
                    mov,
                    score_fn.score(temp_board)
                        * match side {
                            Color::White => 1.0,
                            Color::Black => -1.0,
                        },
                )
            })
            .collect()
    }
}

fn get_sorted_moves(board: &Board, side: &Color) -> Vec<Move> {
    let mut checkers = Vec::with_capacity(4);
    let mut promotions = Vec::with_capacity(4);
    let mut captures = Vec::with_capacity(20);
    let mut moves = Vec::with_capacity(24);

    board.generate_moves_for(board.checkers(), |piece_moves| {
        checkers.extend(piece_moves);
        false
    });

    board.generate_moves_for(board.colors(*side), |piece_moves| {
        for mov in piece_moves {
            if mov.promotion.is_some() {
                promotions.push(mov);
                return false;
            }
            if let Some(color) = board.color_on(mov.to) {
                if color != *side {
                    captures.push(mov);
                    return false;
                }
            }

            if !checkers.contains(&mov) {
                moves.push(mov);
                return false;
            }
        }
        false
    });

    checkers
        .into_iter()
        .chain(promotions)
        .chain(captures)
        .chain(moves)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algorithm::score::PawnDifferenceScore;
    use cozy_chess::{Board, Square};

    /* #[test]
    fn test_negamax_evaluator() {
        let evaluator = Negamax::<2>::default();

        let board = Board::default();
        let score_fn = PawnDifferenceScore::default();

        let eval = evaluator.eval_moves(board, &score_fn);

        let total_score: f64 = eval.into_iter().map(|(mov, score)| score).sum();
        assert_eq!(total_score, 0.0);

        // pawn from E2 to E4
        let board = Board::from_fen(
            "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1",
            false,
        )
        .unwrap();

        let eval = evaluator.eval_moves(board, &score_fn);

        println!("{:?}", eval);

        let bad_pawn_move = eval
            .iter()
            .filter_map(|(mov, score)| {
                if mov.from == Square::index(52) && mov.to == Square::index(52 - 16) {
                    Some(score)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(bad_pawn_move.len(), 1);
        assert_eq!(bad_pawn_move[0], &-1.0);
    } */
}
