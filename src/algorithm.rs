pub mod choose;
pub mod eval;
pub mod score;

use choose::Chooser;
use cozy_chess::{Board, Move};
use eval::Evaluator;
use score::ScoreFunction;
use std::rc::Rc;

#[derive(Clone)]
pub struct ComputerPlayer {
    algorithm: Rc<dyn Evaluator>,
    score_fn: Rc<dyn ScoreFunction>,
    chooser: Rc<dyn Chooser>,
}

impl ComputerPlayer {
    pub fn new(
        algorithm: Rc<dyn Evaluator>,
        score_fn: Rc<dyn ScoreFunction>,
        chooser: Rc<dyn Chooser>,
    ) -> Self {
        Self {
            algorithm,
            score_fn,
            chooser,
        }
    }

    pub fn get_move(&self, board: Board) -> Option<Move> {
        let eval = self.algorithm.eval_moves(board, self.score_fn.clone());

        let mut moves = Vec::with_capacity(eval.len());
        let mut weights = Vec::with_capacity(eval.len());

        for (mov, score) in eval {
            moves.push(mov);
            weights.push(score);
        }

        let choice = self.chooser.choose(&moves, &weights);

        choice.map(|a| a.clone())
    }

    pub fn change_algorithm(&mut self, algorithm: Rc<dyn Evaluator>) {
        self.algorithm = algorithm;
    }

    pub fn change_score_fn(&mut self, score_fn: Rc<dyn ScoreFunction>) {
        self.score_fn = score_fn;
    }

    pub fn change_chooser(&mut self, chooser: Rc<dyn Chooser>) {
        self.chooser = chooser;
    }
}
