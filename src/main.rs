use yew_router::prelude::*;
use yew::prelude::*;
mod router;
mod pages;
use router::{
    Route,
    switch
};

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
