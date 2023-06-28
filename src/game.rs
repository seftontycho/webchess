use crate::algorithm::{
    choose::GreedyChooser,
    eval::{AlphaBetaNegamax, NaiveEvaluator, Negamax, Negascout},
    score::{piece_value, PawnDifferenceScore},
    ComputerPlayer,
};
use cozy_chess::{Board, Color, GameStatus, Piece, PieceMoves, Square};
use leptos::*;
use std::{collections::HashMap, hash::Hash};

fn map_difference(a: HashMap<Piece, usize>, b: HashMap<Piece, usize>) -> HashMap<Piece, usize> {
    let mut diff = HashMap::new();

    for (piece, count) in a {
        let count_b = b.get(&piece).unwrap_or(&0);
        diff.insert(piece, count - *count_b);
    }

    diff
}

trait Flip {
    fn flip(&mut self);

    fn flipped(&self) -> Self
    where
        Self: Sized + Copy,
    {
        let mut copy = *self;
        copy.flip();
        copy
    }
}

impl Flip for Color {
    fn flip(&mut self) {
        *self = match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone)]
struct MovePicker {
    to: Option<Square>,
    from: Option<Square>,
    promotion: Option<Piece>,
}

impl MovePicker {
    fn new() -> Self {
        Self {
            to: None,
            from: None,
            promotion: None,
        }
    }

    fn set_square(&mut self, square: Square) {
        if self.from.is_some() {
            if self.to.is_some() {
                self.from = Some(square);
                self.to = None;
            } else {
                self.to = Some(square);
            }
        } else {
            self.from = Some(square);
        }
    }

    fn set_promotion(&mut self, promotion: Piece) {
        self.promotion = Some(promotion);
    }

    fn from(&self) -> Option<Square> {
        self.from
    }

    fn to(&self) -> Option<Square> {
        self.to
    }

    fn clear(&mut self) {
        self.from = None;
        self.to = None;
        self.promotion = None;
    }
}

fn format_board_status(board: ReadSignal<Board>) -> String {
    let board = board.get();
    let status = board.status();

    match (status, board.side_to_move()) {
        (GameStatus::Drawn, _) => return "Draw!".to_string(),
        (GameStatus::Ongoing, side) => {
            return format!(
                "{} to move!",
                match side {
                    Color::White => "White",
                    Color::Black => "Black",
                }
            );
        }
        (_, side) => {
            return format!(
                "{} wins!",
                match side.flipped() {
                    Color::White => "White",
                    Color::Black => "Black",
                }
            );
        }
    }
}

fn piece_to_img_path(colour: Option<Color>, piece: Option<Piece>) -> String {
    let color = match colour {
        Some(Color::White) => "w",
        Some(Color::Black) => "b",
        None => return "".to_string(),
    };

    let piece = match piece {
        Some(Piece::Pawn) => "p",
        Some(Piece::Knight) => "n",
        Some(Piece::Bishop) => "b",
        Some(Piece::Rook) => "r",
        Some(Piece::Queen) => "q",
        Some(Piece::King) => "k",
        None => return "".to_string(),
    };

    format!("images/pieces/{}{}.png", color, piece)
}

fn filter_moves(
    moves_from: PieceMoves,
    picker: ReadSignal<MovePicker>,
    moves: &mut Vec<cozy_chess::Move>,
) -> bool {
    let picker = picker.get();

    for mov in moves_from {
        if let Some(to) = picker.to() {
            if let Some(promotion) = picker.promotion {
                if mov.to == to && mov.promotion == Some(promotion) {
                    moves.push(mov);
                }
            } else if mov.to == to {
                moves.push(mov);
            }
        } else {
            moves.push(mov);
        }
    }

    false
}

#[component]
pub fn ChessBoard(cx: Scope, opponent: ReadSignal<ComputerPlayer>) -> impl IntoView {
    let (board, set_board) = create_signal(cx, Board::startpos());

    let all_pieces_white = HashMap::from([
        (Piece::Pawn, 8),
        (Piece::Knight, 2),
        (Piece::Bishop, 2),
        (Piece::Rook, 2),
        (Piece::Queen, 1),
        (Piece::King, 1),
    ]);

    let all_pieces_black = all_pieces_white.clone();

    let (picker, set_picker) = create_signal(cx, MovePicker::new());
    let (user_color, set_user_color) = create_signal(cx, Color::White);

    let color = create_memo(cx, move |_| board.get().side_to_move());

    let moves = create_memo(cx, move |_| {
        let board = board.get();
        let mut moves = Vec::new();

        if let Some(from) = picker.get().from() {
            board.generate_moves_for(from.bitboard(), |moves_from| {
                filter_moves(moves_from, picker, &mut moves)
            });
        };

        moves
    });

    create_effect(cx, move |_| {
        if color.get() == user_color.get() {
            if (moves.get().len() == 1) & picker.get().to().is_some() {
                let mov = moves.get()[0];

                log!("User Playing {:?}", mov);

                cx.batch(|| {
                    set_board.update(|b| b.play(mov));
                    set_picker.update(|p| p.clear());
                });
            }
        }
    });

    create_effect(cx, move |_| {
        if color.get() != user_color.get() {
            if let Some(mov) = opponent.get_untracked().get_move(board.get_untracked()) {
                log!("Opponent playing {:?}", mov);

                cx.batch(|| {
                    set_board.update(|b| b.play(mov));
                    set_picker.update(|p| p.clear());
                });
            }
        }
    });

    let white_captured = create_memo(cx, move |_| {
        let board = board.get();
        let mut white = HashMap::new();

        board.colors(Color::White).into_iter().for_each(|square| {
            let piece = board.piece_on(square).expect("Should be piece here");
            let count = white.entry(piece).or_insert(0);
            *count += 1;
        });

        let white = map_difference(all_pieces_white.clone(), white);
        let mut white = white
            .into_iter()
            .flat_map(|(piece, count)| {
                let mut v = Vec::new();
                for _ in 0..count {
                    v.push(piece);
                }
                v
            })
            .collect::<Vec<_>>();

        white.sort_by(|a, b| piece_value(*a).partial_cmp(&piece_value(*b)).unwrap());

        white
    });

    let black_captured = create_memo(cx, move |_| {
        let board = board.get();
        let mut black = HashMap::new();

        board.colors(Color::Black).into_iter().for_each(|square| {
            let piece = board.piece_on(square).expect("Should be piece here");
            let count = black.entry(piece).or_insert(0);
            *count += 1;
        });

        let black = map_difference(all_pieces_black.clone(), black);
        let mut black = black
            .into_iter()
            .flat_map(|(piece, count)| {
                let mut v = Vec::new();
                for _ in 0..count {
                    v.push(piece);
                }
                v
            })
            .collect::<Vec<_>>();

        black.sort_by(|a, b| piece_value(*a).partial_cmp(&piece_value(*b)).unwrap());

        black
    });

    let needs_promotion = create_memo(cx, move |_| {
        if color.get() != user_color.get() {
            return false;
        }
        let moves = moves.get();
        moves.len() > 0 && moves.into_iter().all(|mov| mov.promotion.is_some())
    });

    view! { cx,
        <div class="flex justify-center">
            <div>
                <button class="m-5 bg-chess-green text-white font-bold rounded-md p-2 hover:bg-chess-white hover:text-chess-green"
                on:click=move |_| {
                        set_board.set(Board::startpos());
                        log!("Board reset");
                }>
                    "New Game"
                </button>
            </div>
            <div>
                <button class="m-5 bg-chess-green text-white font-bold rounded-md p-2 hover:bg-chess-white hover:text-chess-green"
                on:click=move |_| {
                    cx.batch(|| {
                        set_user_color.update(|c| c.flip());
                        log!("Colour changed to {}", user_color.get_untracked());
                        set_board.set(Board::startpos());
                        log!("Board reset");
                    })
                }>
                    {match user_color.get() {
                        Color::White => "Play as Black",
                        Color::Black => "Play as White",
                    }}
                </button>
            </div>
            <div class="m-5 text-center text-white font-bold p-2 rounded-md bg-chess-green">
                {move || format_board_status(board)}
            </div>
        </div>
        <div class="flex justify-center mx-auto max-w-2xl h-4">
            {
                move || {
                    let pieces = match user_color.get().flipped() {
                        Color::White => white_captured.get(),
                        Color::Black => black_captured.get(),
                    };

                    pieces.into_iter().map(|piece| {
                        view! { cx,
                            <img class="p-0 max-w-piece" src=piece_to_img_path(Some(user_color.get()), Some(piece))/>
                        }
                    }).collect::<Vec<_>>()
                }
            }
        </div>
        <div class="max-w-2xl mx-auto my-auto">
            <div>
                <div class="select-none grid grid-cols-8 mx-auto border-2 border-black">
                    {move || (0..64)
                        .map(|i| {
                            let square = match user_color.get() {
                                Color::White => Square::index(i).flip_rank(),
                                Color::Black => Square::index(i),
                            };
                            view! { cx, <Square square=square board=board picker=picker set_picker=set_picker/> }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
            <div>
                <Show
                    when=move || needs_promotion.get()
                    fallback= |_| {}
                >
                    <div class="my-5 mx-auto max-w-fit grid grid-cols-5">
                        {vec![Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight, Piece::Pawn,]
                            .into_iter()
                            .map(|p| {
                                let path = piece_to_img_path(Some(color.get()), Some(p));
                                view! { cx,
                                    <div
                                        class=format!(
                                            "aspect-square bg-chess-{} hover:shadow-square-inner", match color.get() {
                                            Color::White => "green", Color::Black => "white", }
                                        )
                                        on:click=move |_| { set_picker.update(|picker| { picker.set_promotion(p) }) }
                                    >
                                        <img class="p-0 max-w-piece" src=path/>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn Square(
    cx: Scope,
    board: ReadSignal<Board>,
    square: Square,
    picker: ReadSignal<MovePicker>,
    set_picker: WriteSignal<MovePicker>,
) -> impl IntoView {
    let color = if (square.rank() as usize + square.file() as usize) % 2 == 0 {
        "bg-chess-green"
    } else {
        "bg-chess-white"
    };

    view! { cx,
        <div
            class=move || {
                let highlight = if picker.get().from() == Some(square) {
                    "shadow-square-inner shadow-green-500"
                } else if picker.get().to() == Some(square) {
                    "shadow-square-inner shadow-yellow-500"
                } else {
                    ""
                };
                if highlight == "" {
                    format!("select-none aspect-square {} hover:shadow-square-inner", color)
                } else {
                    format!("select-none aspect-square {} {}", color, highlight)
                }
            }
            on:click=move |_| { set_picker.update(|p| p.set_square(square)) }
        >
            <Show when=move || { board.get().piece_on(square).is_some() } fallback=|_| {}>
                <img
                    class="p-0 max-w-piece"
                    src=move || {
                        let color = board.get().color_on(square);
                        let piece = board.get().piece_on(square);
                        piece_to_img_path(color, piece)
                    }
                />
            </Show>
        </div>
    }
}
