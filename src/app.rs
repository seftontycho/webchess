use crate::game::ChessBoard;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="min-h-screen bg-page-background">
            <div class="my-0 mx-auto py-10 max-w-3xl text-center text-page-text text-3xl">
                "Web Chess"
            </div>
            <ChessBoard/>
        </div>
    }
}
