use crate::game::ChessBoard;
use leptos::*;
use leptos_meta::*;

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
    view! { cx,
        <div class="min-h-screen bg-page-background">
            <ChessBoard/>
        </div>
    }
}
