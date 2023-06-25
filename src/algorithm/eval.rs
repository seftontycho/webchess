use super::score::ScoreFunction;
use cozy_chess::{Board, Color, GameStatus, Move};

pub trait Evaluator {
    fn eval_moves<T: ScoreFunction>(&self, board: Board, score_fn: &T) -> Vec<(Move, f64)>;
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
                    score_fn.score(&temp_board)
                        * match side {
                            Color::White => 1.0,
                            Color::Black => -1.0,
                        },
                )
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct Negamax {
    depth: usize,
}

impl Default for Negamax {
    fn default() -> Self {
        Self { depth: 2 }
    }
}

impl Negamax {
    pub fn new(depth: usize) -> Self {
        Self { depth }
    }

    fn negamax<T: ScoreFunction>(
        &self,
        score_fn: &T,
        board: Board,
        depth: usize,
        negative: bool,
    ) -> f64 {
        if depth == 0 {
            return match negative {
                true => -score_fn.score(&board),
                false => score_fn.score(&board),
            };
        };

        let side = board.side_to_move();

        match board.status() {
            GameStatus::Drawn => return 0.0,
            GameStatus::Won => {
                return match negative {
                    true => -1.0,
                    false => 1.0,
                } * match side {
                    Color::White => -1000.0,
                    Color::Black => 1000.0,
                };
            }
            GameStatus::Ongoing => {}
        };

        let moves = get_sorted_moves(&board, &side);

        /* moves
        .into_iter()
        .map(|mov| {
            let mut temp_board = board.clone();
            temp_board.play(mov);
            -self.negamax(score_fn, temp_board, depth - 1, !negative)
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0) */

        let mut best_score = f64::NEG_INFINITY;

        for mov in moves {
            let mut temp_board = board.clone();
            temp_board.play(mov);
            best_score = best_score.max(-self.negamax(score_fn, temp_board, depth - 1, !negative));
        }

        best_score
    }
}

impl Evaluator for Negamax {
    fn eval_moves<T: ScoreFunction>(&self, board: Board, score_fn: &T) -> Vec<(Move, f64)> {
        let side = board.side_to_move();
        let moves = get_sorted_moves(&board, &side);

        // white => next turn is black => negative
        let negative = match side {
            Color::White => false,
            Color::Black => true,
        };

        moves
            .into_iter()
            .map(|mov| {
                let mut temp_board = board.clone();
                temp_board.play(mov);
                (
                    mov,
                    -self.negamax(score_fn, temp_board, self.depth - 1, !negative),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algorithm::score::PawnDifferenceScore;
    use cozy_chess::{Board, Square};

    #[test]
    fn test_negamax_evaluator() {
        let evaluator = Negamax::new(3);

        let board = Board::default();
        let score_fn = PawnDifferenceScore::default();

        let eval = evaluator.eval_moves(board, &score_fn);

        let total_score: f64 = eval.into_iter().map(|(mov, score)| score).sum();
        assert_eq!(total_score, 0.0);

        let mut board = Board::startpos();

        let mov = Move {
            from: Square::D2,
            to: Square::D4,
            promotion: None,
        };

        board.play(mov);

        let eval = evaluator.eval_moves(board, &score_fn);

        println!("{:?}", eval);
        let nonzero_scores = eval
            .iter()
            .filter_map(|(mov, score)| match score {
                0.0 => None,
                _ => Some(score),
            })
            .collect::<Vec<_>>();

        println!("Scores: {:?}", nonzero_scores);
        assert_eq!(nonzero_scores.len(), 2);

        // the two (2-square) pawn moves either side of the white pawn
        // should result in -1.0 for black as white can take next go
        assert_eq!(nonzero_scores, vec![&-1.0f64, &-1.0f64]);
    }
}
