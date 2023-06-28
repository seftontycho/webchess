use crate::algorithm::{
    self, choose::GreedyChooser, eval::Negamax, score::PawnDifferenceScore, ComputerPlayer,
};
use leptos::*;

#[component]
pub fn Mixer(cx: Scope, set_opponent: WriteSignal<ComputerPlayer>) -> impl IntoView {
    view! { cx,
    <aside id="sidebar-multi-level-sidebar" class="fixed top-0 left-0 z-40 w-1/3 h-screen transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
    <div class="h-full px-3 py-4 overflow-y-auto bg-chess-green">
       <ul class="space-y-2 font-medium">
          <li>
             <button type="button" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700" aria-controls="dropdown-example" data-collapse-toggle="dropdown-example">
                   <span class="flex-1 ml-3 text-left whitespace-nowrap" sidebar-toggle-item>"OUTER CONTENT HERE"</span>
                   <svg sidebar-toggle-item class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
             </button>
             <ul id="dropdown-example" class="hidden py-2 space-y-2">
                   <li>
                   <a class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">"Products"</a>
                   </li>
             </ul>
          </li>
       </ul>
    </div>
    </aside>
    }
}
