use cozy_chess::{Board, Color, Move, Piece};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub fn calc_best_move(board: Board) -> Move {
    let mut moves = Vec::new();
    let side = board.side_to_move();

    board.generate_moves_for(board.colors(side), |piece_moves| {
        moves.extend(piece_moves);
        false
    });

    let scores = moves
        .iter()
        .map(|&m| {
            let mut temp_board = board.clone();
            temp_board.play(m);
            board_score(&temp_board, side).tanh() + 1.0
        })
        .collect::<Vec<_>>();

    let score_sum = scores.iter().sum::<f64>();

    let scores: Vec<_> = scores.into_iter().map(|s| s / score_sum).collect();

    let dist = WeightedIndex::new(&scores).unwrap();
    let mut rng = thread_rng();

    moves[dist.sample(&mut rng)]
}

fn board_score(board: &Board, color: Color) -> f64 {
    let mut score = 0.0;

    for square in board.occupied() {
        let piece = board.piece_on(square).unwrap();
        let piece_score = piece_value(piece);

        score += if board.color_on(square).unwrap() == color {
            piece_score
        } else {
            -piece_score
        };
    }

    score
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
