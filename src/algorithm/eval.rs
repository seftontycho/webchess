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
                continue;
            }
            if let Some(color) = board.color_on(mov.to) {
                if color != *side {
                    captures.push(mov);
                    continue;
                }
            }

            if !checkers.contains(&mov) {
                moves.push(mov);
                continue;
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

    fn negamax<T: ScoreFunction>(score_fn: &T, board: Board, depth: usize, negative: bool) -> f64 {
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

        moves
            .into_iter()
            .map(|mov| {
                let mut temp_board = board.clone();
                temp_board.play(mov);
                -Self::negamax(score_fn, temp_board, depth - 1, !negative)
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}

impl Evaluator for Negamax {
    fn eval_moves<T: ScoreFunction>(&self, board: Board, score_fn: &T) -> Vec<(Move, f64)> {
        let side = board.side_to_move();
        let moves = get_sorted_moves(&board, &side);

        // white => next turn is black => negative should be true
        let negative = match side {
            Color::White => true,
            Color::Black => false,
        };

        moves
            .into_iter()
            .map(|mov| {
                let mut temp_board = board.clone();
                temp_board.play(mov);
                (
                    mov,
                    -Self::negamax(score_fn, temp_board, self.depth - 1, negative),
                )
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct AlphaBetaNegamax {
    depth: usize,
}

impl Default for AlphaBetaNegamax {
    fn default() -> Self {
        Self { depth: 2 }
    }
}

impl AlphaBetaNegamax {
    pub fn new(depth: usize) -> Self {
        Self { depth }
    }

    fn negamax<T: ScoreFunction>(
        score_fn: &T,
        board: Board,
        depth: usize,
        alpha: f64,
        beta: f64,
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

        let mut best_score = f64::NEG_INFINITY;
        let mut alpha = alpha;

        for mov in get_sorted_moves(&board, &side) {
            let mut temp_board = board.clone();
            temp_board.play(mov);
            best_score = best_score.max(-Self::negamax(
                score_fn,
                temp_board,
                depth - 1,
                -beta,
                -alpha,
                !negative,
            ));
            alpha = alpha.max(best_score);

            if alpha >= beta {
                break;
            };
        }

        best_score
    }
}

impl Evaluator for AlphaBetaNegamax {
    fn eval_moves<T: ScoreFunction>(&self, board: Board, score_fn: &T) -> Vec<(Move, f64)> {
        let side = board.side_to_move();
        let moves = get_sorted_moves(&board, &side);

        // white => next turn is black => negative should be true
        let negative = match side {
            Color::White => true,
            Color::Black => false,
        };

        moves
            .into_iter()
            .map(|mov| {
                let mut temp_board = board.clone();
                temp_board.play(mov);
                (
                    mov,
                    -Self::negamax(
                        score_fn,
                        temp_board,
                        self.depth - 1,
                        f64::NEG_INFINITY,
                        f64::INFINITY,
                        negative,
                    ),
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

    fn get_all_moves(board: &Board, side: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        board.generate_moves_for(board.colors(*side), |piece_moves| {
            moves.extend(piece_moves);
            false
        });
        moves
    }

    #[test]
    fn test_startpos_get_sorted_moves() {
        let sorter = |mov: &Move| {
            100_000 * mov.from as u64
                + 1_000 * mov.to as u64
                + match mov.promotion {
                    Some(piece) => piece as u64,
                    None => 0,
                } as u64
        };

        //test for startpos
        let board = Board::startpos();
        let side = board.side_to_move();

        let mut expected = get_all_moves(&board, &side);
        let mut moves = get_sorted_moves(&board, &side);

        expected.sort_by_key(sorter);
        moves.sort_by_key(sorter);

        assert_eq!(moves.len(), expected.len());
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_game_get_sorted_moves() {
        let sorter = |mov: &Move| {
            100_000 * mov.from as u64
                + 1_000 * mov.to as u64
                + match mov.promotion {
                    Some(piece) => piece as u64,
                    None => 0,
                } as u64
        };

        //test for startpos
        let mut board = Board::startpos();
        let game_moves = vec!["d2d4", "e7e5", "d4e5", "d8g5", "e5e6", "g5c1"];

        for mov in game_moves {
            println!("Move: {}", mov);
            board.play(mov.parse().unwrap());

            let side = board.side_to_move();
            println!("Side to move: {:?}", side);

            let mut expected = get_all_moves(&board, &side);
            let mut moves = get_sorted_moves(&board, &side);

            expected.sort_by_key(sorter);
            moves.sort_by_key(sorter);

            assert_eq!(moves.len(), expected.len(), "Moves vs Expected (lengths)");

            for (m, e) in moves.iter().zip(expected.iter()) {
                println!("Move: {:?}, Expected: {:?}", m, e);
                assert_eq!(m, e);
            }
        }
    }

    #[test]
    fn startpos_test_negamax_evaluator() {
        let evaluator = Negamax::new(2);

        let board = Board::startpos();
        let score_fn = PawnDifferenceScore::default();

        let eval = evaluator.eval_moves(board, &score_fn);

        let total_score: f64 = eval.into_iter().map(|(mov, score)| score).sum();
        assert_eq!(total_score, 0.0);
    }

    #[test]
    fn pawn_move_test_negamax() {
        let evaluator = Negamax::new(2);
        let score_fn = PawnDifferenceScore::default();

        let mut board = Board::startpos();

        let mov = Move {
            from: Square::D2,
            to: Square::D4,
            promotion: None,
        };

        board.play(mov);

        let eval = evaluator.eval_moves(board, &score_fn);

        println!("{:?}", eval);
        let mut nonzero_scores = eval
            .iter()
            .filter_map(|(mov, score)| match score {
                0.0 => None,
                _ => Some(score),
            })
            .collect::<Vec<_>>();

        println!("Scores: {:?}", nonzero_scores);
        assert_eq!(nonzero_scores.len(), 5);

        // C7C5, E7E5, G7G5, H7H6 all have score -1.0 as all present pawn captures
        // G8H6 has score -3.0 as it presents a knigth capture
        let expected: Vec<&f64> = vec![&-3.0, &-1.0, &-1.0, &-1.0, &-1.0];

        nonzero_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(nonzero_scores, expected);
    }

    #[test]
    fn test_alphabetanegamax_vs_negamax() {
        let expected_evaluator = Negamax::new(4);
        let evaluator = AlphaBetaNegamax::new(4);
        let score_fn = PawnDifferenceScore::default();

        let mut board = Board::startpos();

        let mov = Move {
            from: Square::D2,
            to: Square::D4,
            promotion: None,
        };

        board.play(mov);

        let expected_eval = expected_evaluator.eval_moves(board.clone(), &score_fn);
        let eval = evaluator.eval_moves(board, &score_fn);

        assert_eq!(
            eval.len(),
            expected_eval.len(),
            "Eval vs Expected (lengths)"
        );

        for (e, ee) in eval.iter().zip(expected_eval.iter()) {
            assert_eq!(e, ee, "Zipped evals should be equal");
        }
    }
}
