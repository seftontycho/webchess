use crate::{
    algorithm::{
        self, choose::GreedyChooser, eval::Negamax, score::PawnDifferenceScore, ComputerPlayer,
    },
    game::ChessBoard,
    mixer::Mixer,
};
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
    let algorithm = Rc::new(Negamax::new(3));
    let score = Rc::new(PawnDifferenceScore::default());
    let chooser = Rc::new(GreedyChooser::default());

    let (opponent, set_opponent) =
        create_signal(cx, ComputerPlayer::new(algorithm, score, chooser));

    view! { cx,
        <div class="min-h-screen bg-page-background">
            <ChessBoard opponent=opponent/>
        </div>
    }
}
