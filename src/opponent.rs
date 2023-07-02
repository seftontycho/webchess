use std::rc::Rc;

use crate::algorithm::{choose::*, eval::*, score::*, ComputerPlayer};
use leptos::*;

#[component]
pub fn OpponentMaker(cx: Scope) -> impl IntoView {
    let set_opponent =
        use_context::<WriteSignal<ComputerPlayer>>(cx).expect("should be opponent here");
    // need to add every eval, score_fn, chooser here

    let (current_eval, set_current_eval) = create_signal(cx, "1 Move Lookahead");
    let (current_score, set_current_score) = create_signal(cx, "Pawn Difference Score");

    view! { cx,
        <div class="m-4 h-full">
            <div class="text-4xl font-bold text-page-text mb-4">
                "Opponent Settings"
            </div>
            <div class="grid grid-cols-3 h-full gap-4 grid-flow-col">
                <div class="grid grid-rows-6 gap-4">
                    <div>
                        <button class=move || {
                            let highlight = current_eval.get() == "1 Move Lookahead";
                            format!("{} {} text-3xl border-2 border-page-dark rounded-md w-full h-full hover:text-white hover:bg-page-dark", match highlight {
                                true => "text-page-textinverse",
                                false => "text-page-text",
                            }, match highlight {
                                true => "bg-page-text",
                                false => "",
                            })
                        }
                        on:click=move |_| {
                            set_opponent.update(|player| player.change_algorithm(Rc::new(NaiveEvaluator::default())));
                            set_current_eval.set("1 Move Lookahead");
                            log!("Opponent set to NaiveEvaluator");
                        }>
                            "1 Move Lookahead"
                        </button>
                    </div>

                    <div>
                        <button class=move || {
                            let highlight = current_eval.get() == "Negamax";
                            format!("{} {} text-3xl border-2 border-page-dark rounded-md w-full h-full hover:text-white hover:bg-page-dark", match highlight {
                                true => "text-page-textinverse",
                                false => "text-page-text",
                            }, match highlight {
                                true => "bg-page-text",
                                false => "",
                            })
                        }
                        on:click=move |_| {
                            set_opponent.update(|player| player.change_algorithm(Rc::new(Negamax::default())));
                            set_current_eval.set("Negamax");
                            log!("Opponent set to Negamax");
                        }>
                            "Negamax"
                        </button>
                    </div>

                    <div>
                    <button class=move || {
                        let highlight = current_eval.get() == "Negamax with Alpha-Beta Pruning";
                        format!("{} {} text-3xl border-2 border-page-dark rounded-md w-full h-full hover:text-white hover:bg-page-dark", match highlight {
                            true => "text-page-textinverse",
                            false => "text-page-text",
                        }, match highlight {
                            true => "bg-page-text",
                            false => "",
                        })
                    }
                    on:click=move |_| {
                        set_opponent.update(|player| player.change_algorithm(Rc::new(Negamax::default())));
                        set_current_eval.set("Negamax with Alpha-Beta Pruning");
                        log!("Opponent set to AlphaBetaNegamax");
                    }>
                    "Negamax with Alpha-Beta Pruning"
                    </button>
                    </div>
                </div>

                <div class="grid grid-rows-6 gap-4">
                    <div>
                        <button class=move || {
                            let highlight = current_score.get() == "Pawn Difference Score";
                            format!("{} {} text-3xl border-2 border-page-dark rounded-md w-full h-full hover:text-white hover:bg-page-dark", match highlight {
                                true => "text-page-textinverse",
                                false => "text-page-text",
                            }, match highlight {
                                true => "bg-page-text",
                                false => "",
                            })
                        }
                        on:click=move |_| {
                            set_opponent.update(|player| player.change_score_fn(Rc::new(PawnDifferenceScore::default())));
                            set_current_score.set("Pawn Difference Score");
                            log!("Opponent set to PawnDifferenceScore");
                        }>
                        "Pawn Difference Score"
                        </button>
                    </div>
                </div>







            </div>
        </div>


    }
}
