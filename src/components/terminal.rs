use crate::router::Route;
use web_sys::{HtmlInputElement, Window};
use yew::{
    function_component, html, use_effect_with, use_node_ref, use_state, Callback, Html,
    KeyboardEvent,
};
use yew_router::hooks::use_navigator;

#[function_component]
pub fn Terminal() -> Html {
    let input_terminal = use_node_ref();
    let input_terminal_clone = input_terminal.clone();
    let logs = use_state(|| Vec::<String>::new());
    let navigation = use_navigator().unwrap();

    let window = web_sys::window();

    use_effect_with((), move |_| {
        let input_clone = &input_terminal_clone.clone();
        if let Some(input_node) = input_clone.cast::<HtmlInputElement>() {
            _ = input_node.focus();
        };
    });

    let click_on_terminal_box = {
        let input_clone_for_box = input_terminal.clone();
        Callback::from(move |_| {
            let input_clone = &input_clone_for_box.clone();
            if let Some(input_node) = input_clone.cast::<HtmlInputElement>() {
                _ = input_node.focus();
            };
        })
    };

    let onkeypress = {
        let logs_clone = logs.clone();
        let input_clone_for_log = input_terminal.clone();
        let navigation_clone = navigation.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                let mut new_logs = (*logs_clone).clone();
                if let Some(input_node) = &input_clone_for_log.cast::<HtmlInputElement>() {
                    match input_node.value().as_str() {
                        "clear" => logs_clone.set(Vec::<String>::new()),
                        "cd .." => navigation_clone.push(&Route::Mode),
                        "cd /view/standard" => navigation_clone.push(&Route::Standard),
                        "exit" => navigation_clone.push(&Route::Mode),
                        _ => {
                            new_logs.push(input_node.value());
                            logs_clone.set(new_logs);
                        }
                    }
                    input_node.set_value("");
                }
            }
        })
    };

    html! {
      <div
        class="w-[98vw] h-[98vh] m-h-[98vh] [content-visibility:auto] rounded-xl mx-4 my-2 p-4 flex flex-col gap-y-2 bg-zinc-800 cursor-text overflow-y-auto scroll-smooth"
        onclick={click_on_terminal_box}
      >
        { for (*logs).iter().map(|log_entry| html! { <div>{">> "} {log_entry} </div>})}
        <div class="flex flex-row w-full">
          <span class="w-1/12">{"guess@DA2Mdev:-$"}</span>
          <input
            type="text"
            class="inline-block outline-none w-11/12"
            id="terminal_input"
            autofocus=true ref={input_terminal}
            {onkeypress}
          />
        </div>
      </div>
    }
}
