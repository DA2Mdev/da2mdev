use yew::prelude::*;
mod components;
use components::{
    great::Great,
};

#[function_component]

fn App() -> Html {

  html! {
    <div>
      <Great />
    </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
