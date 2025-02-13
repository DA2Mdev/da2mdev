use crate::router::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component]
pub fn Mode() -> Html {
    html! {
      <div class="flex flex-row items-center justify-center gap-5">
        <Link<Route>
         to={Route::Terminal}
         classes="w-60 h-60 p-5 border border-zinc-100 rounded-lg flex flex-col items-center justify-center text-center dark:hover:bg-zinc-700 dark:hover:shadow-sm dark:hover:shadow-zinc-700 transition"
        >
          <img class="size-18" src="img/terminal-dark.svg" alt="terminal" />
          <span class="font-semibold text-xl -m-2">{"Terminal"}</span>
          <span class="mt-4">{"In this mode we can interact with the web like a Linux terminal, using commands."}</span>
        </Link<Route>>
        <Link<Route>
         to={Route::Standard}
         classes="w-60 h-60 p-5 border border-zinc-100 rounded-lg flex flex-col items-center justify-center text-center dark:hover:bg-zinc-700 dark:hover:shadow-sm dark:hover:shadow-zinc-700 transition"
        >
          <img class="size-12 m-3 mb-0" src="img/view-list-details-dark.svg" alt="standard" />
          <span class="font-semibold text-xl">{"Standard"}</span>
          <span class="mt-4">{"In standard mode we can see a normal view if you don't have experience with the Linux terminal."}</span>
        </Link<Route>>
      </div>
    }
}
