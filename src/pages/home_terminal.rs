use crate::components::terminal::Terminal;
use yew::{function_component, html, Html};

#[function_component]
pub fn HomeTerminal() -> Html {
    html! {
      <div>
        <Terminal />
      </div>
    }
}
