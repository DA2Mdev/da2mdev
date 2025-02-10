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
      <BrowserRouter>
        <Switch<Route> render={switch} />
      </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
