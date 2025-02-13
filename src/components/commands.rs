use crate::utils::commands::COMMANDS;
use yew::{function_component, html, Html};

#[function_component(Commands)]
pub fn commands() -> Html {
    html! {
        <div class="flex flex-wrap gap-x-20 text-green-500">
            {for COMMANDS.iter().map(|command| html! { <span> {command.to_string()} </span> })}
        </div>
    }
}
