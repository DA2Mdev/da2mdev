use crate::router::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component]
pub fn Mode() -> Html {
    html! {
      <div class="flex flex-row items-center justify-center gap-5">
        <Link<Route> to={Route::Terminal} classes="w-40 h-50 border border-zinc-100 rounded-lg text-center">
          {"Go to terminal view"}
        </Link<Route>>
        <Link<Route> to={Route::Standard} classes="w-40 h-50 border border-zinc-100 rounded-lg text-center">
          {"Go to standard view"}
        </Link<Route>>
      </div>
    }
}
