use crate::{
    algorithm::{
        choose::GreedyChooser,
        eval::{AlphaBetaNegamax, Evaluator},
        score::PawnDifferenceScore,
        ComputerPlayer,
    },
    game::{ChessBoard, Flip},
    opponent::OpponentMaker,
};
use cozy_chess::{Board, Color, GameStatus};
use leptos::*;
use leptos_meta::*;
use std::rc::Rc;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Home/>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (board, set_board) = create_signal(cx, Board::startpos());
    provide_context(cx, board);
    provide_context(cx, set_board);

    let (user_color, set_user_color) = create_signal(cx, Color::White);
    provide_context(cx, user_color);
    provide_context(cx, set_user_color);

    let (opponent, set_opponent) = create_signal(
        cx,
        ComputerPlayer::new(
            Rc::new(AlphaBetaNegamax::new(4)),
            Rc::new(PawnDifferenceScore::default()),
            Rc::new(GreedyChooser::default()),
        ),
    );
    provide_context(cx, opponent);
    provide_context(cx, set_opponent);

    view! { cx,
        <div class="parent text-center flex flex-col h-screen bg-page-background">
            <main class="bg-page-background flex-1 flex">
                <LeftBar/>
                <MainContent/>
            </main>
        </div>
    }
}

#[component]
pub fn LeftBar(cx: Scope) -> impl IntoView {
    let set_board = use_context::<WriteSignal<Board>>(cx).expect("should be board here");
    let user_color = use_context::<ReadSignal<Color>>(cx).expect("should be color here");
    let set_user_color = use_context::<WriteSignal<Color>>(cx).expect("should be color here");

    view! {cx,
        <aside class="fixed h-full flex bg-page-bar lg:flex flex-shrink-0 flex-col w-56 transition-width duration-75">
            <div class="relative flex-1 flex flex-col min-h-0 pt-0 bg-page-bar">
                <ul>
                    <li class="hover:bg-page-dark">
                        <button class="text-page-text text-3xl font-bold w-full text-left my-4 ml-2 hover:text-white"
                        on:click=move |_| {
                                set_board.set(Board::startpos());
                                log!("Board reset");
                        }>
                            "New Game"
                        </button>
                    </li>
                    <li class="hover:bg-page-dark">
                        <button class="text-page-text text-3xl font-bold w-full text-left my-4 ml-2 hover:text-white"
                        on:click=move |_| {
                            cx.batch(|| {
                                set_user_color.update(|c| c.flip());
                                log!("Colour changed to {}", user_color.get_untracked());
                                set_board.set(Board::startpos());
                                log!("Board reset");
                            })
                        }>
                            {move || match user_color.get() {
                                Color::White => "Play as Black",
                                Color::Black => "Play as White",
                            }}
                        </button>
                    </li>

                </ul>
            </div>
        </aside>
    }
}

#[component]
fn MainContent(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex-1 grid grid-cols-2 bg-page-background lg:ml-56">
            <div class="mt-8 ml-16">
                <ChessBoard/>
            </div>
            <div class="my-8 ml-8 mr-48 bg-page-bar rounded-[1rem]">
                <OpponentMaker/>
            </div>
        </div>
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
