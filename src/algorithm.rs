pub mod choose;
pub mod eval;
pub mod score;

use choose::Chooser;
use cozy_chess::{Board, Move};
use eval::Evaluator;
use leptos::log;
use score::ScoreFunction;

#[derive(Clone)]
pub struct ComputerPlayer<E, S, C>
where
    E: Evaluator + Clone,
    S: ScoreFunction + Clone,
    C: Chooser + Clone,
{
    algorithm: E,
    score_fn: S,
    chooser: C,
}

impl<E, S, C> ComputerPlayer<E, S, C>
where
    E: Evaluator + Clone,
    S: ScoreFunction + Clone,
    C: Chooser + Clone,
{
    pub fn new(algorithm: E, score_fn: S, chooser: C) -> Self {
        Self {
            algorithm,
            score_fn,
            chooser,
        }
    }

    pub fn get_move(&self, board: Board) -> Option<Move> {
        let eval = self.algorithm.eval_moves(board, &self.score_fn);

        let mut moves = Vec::with_capacity(eval.len());
        let mut weights = Vec::with_capacity(eval.len());

        for (mov, score) in eval {
            moves.push(mov);
            weights.push(score);
        }

        let choice = self.chooser.choose(&moves, &weights);

        choice.map(|a| a.clone())
    }
}
