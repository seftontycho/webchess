use cozy_chess::{Board, Color, GameStatus, Piece, PieceMoves, Square};
use leptos::*;

#[derive(Clone)]
struct MovePicker {
    to: Option<Square>,
    from: Option<Square>,
    promotion: Option<Piece>,
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
        // IDK why this code is reachable but it is
        None => return "".to_string(),
    };

    let piece = match piece {
        Some(Piece::Pawn) => "p",
        Some(Piece::Knight) => "n",
        Some(Piece::Bishop) => "b",
        Some(Piece::Rook) => "r",
        Some(Piece::Queen) => "q",
        Some(Piece::King) => "k",
        // IDK why this code is reachable but it is
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
pub fn ChessBoard(cx: Scope) -> impl IntoView {
    let (board, set_board) = create_signal(cx, Board::startpos());
    let (picker, set_picker) = create_signal(cx, MovePicker::new());
    let (user_color, set_user_color) = create_signal(cx, Color::White);

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
        if (moves.get().len() == 1) & picker.get().to().is_some() {
            let mov = moves.get()[0];
            set_board.update(|b| b.play(mov));
            set_picker.update(|p| p.clear());
        }
    });

    let needs_promotion = create_memo(cx, move |_| {
        let moves = moves.get();
        moves.len() > 0 && moves.into_iter().all(|mov| mov.promotion.is_some())
    });

    let color = create_memo(cx, move |_| board.get().side_to_move());

    view! { cx,
        <div class="flex justify-center">
            <div>
                <button class="mr-5 my-5 bg-chess-green text-white font-bold rounded-md p-2 hover:bg-chess-white hover:text-chess-green"
                on:click=move |_| {
                        set_board.set(Board::startpos());
                        log!("Board reset");
                }>
                    "New Game"
                </button>
            </div>
            <div>
                <button class="mr-20 my-5 bg-chess-green text-white font-bold rounded-md p-2 hover:bg-chess-white hover:text-chess-green"
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
            <div class="ml-20 my-5 text-center text-white font-bold p-2 rounded-md bg-chess-green">
                {move || format_board_status(board)}
            </div>
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
                    fallback=|c| {
                        view! { c,  }
                    }
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
