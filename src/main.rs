use router::{switch, Route};
pub use yew::prelude::*;
use yew_router::prelude::*;

pub use web_sys;
mod components;
mod pages;
mod router;

#[function_component]
fn App() -> Html {
    html! {
      <main class="w-screen h-screen bg-zinc-100 dark:bg-zinc-900 grid text-zinc-900 dark:text-zinc-100">
      <BrowserRouter>
        <Switch<Route> render={switch} />
      </BrowserRouter>
      </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
