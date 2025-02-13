use crate::router::Route;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect_with, use_node_ref, use_state, Callback, Html,
    KeyboardEvent, UseStateHandle,
};
use yew_router::hooks::use_navigator;

fn handle_clear_logs(logs: &UseStateHandle<Vec<String>>) {
    logs.set(Vec::<String>::new());
}

// fn handle_clear_logs(navigation: &UseStateHandle<Vec<Route>>, route: Route) {
//     navigation.push(route);
// }

#[function_component]
pub fn Terminal() -> Html {
    let input_terminal = use_node_ref();
    let input_terminal_clone = input_terminal.clone();
    let logs = use_state(|| Vec::<String>::new());
    let navigation = use_navigator().unwrap();
    let tab_pressed = use_state(|| false);
    let ctrl_pressed = use_state(|| false);
    let show_commands = use_state(|| false);

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

    let sc = {
        let ctrl_pressed_clone = ctrl_pressed.clone();
        let logs_clone = logs.clone();
        Callback::from(move |e: KeyboardEvent| {
            let ctrl_pressed_clone = ctrl_pressed_clone.clone();
            let logs_clone = logs_clone.clone();
            match e.key_code() {
                17 => ctrl_pressed_clone.set(true),
                76 if *ctrl_pressed_clone => logs_clone.set(Vec::<String>::new()),
                _ => {}
            }
        })
    };

    let onkeydown = {
        let tab_pressend_clone = tab_pressed.clone();
        let sc_clone = sc.clone();
        let show_commands_clone = show_commands.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key_code() == 76 && e.ctrl_key() {
                e.prevent_default();
            }
            sc_clone.emit(e.clone());
            if e.key() == "Tab" {
                if *tab_pressed {
                    show_commands_clone.set(true);
                }
                tab_pressend_clone.set(true);
                let tab_pressend_clone = tab_pressed.clone();
                gloo::timers::callback::Timeout::new(400, move || tab_pressend_clone.set(false))
                    .forget();
                e.prevent_default();
            }
        })
    };

    let onkeypress = {
        let logs_clone = logs.clone();
        let input_clone_for_log = input_terminal.clone();
        let navigation_clone = navigation.clone();
        let show_commands_clone = show_commands.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                show_commands_clone.set(false);
                let mut new_logs = (*logs_clone).clone();
                if let Some(input_node) = &input_clone_for_log.cast::<HtmlInputElement>() {
                    match input_node.value().as_str() {
                        "clear" => handle_clear_logs(&logs_clone),
                        "cd .." => navigation_clone.push(&Route::Mode),
                        "cd /view/standard" => navigation_clone.push(&Route::Standard),
                        "exit" => navigation_clone.push(&Route::Mode),
                        _ => {
                            if !*show_commands_clone || !input_node.value().is_empty() {
                                new_logs.push(input_node.value());
                                logs_clone.set(new_logs);
                            }
                        }
                    }
                    input_node.set_value("");
                }
            }
        })
    };

    let onkeyup = {
        let ctrl_pressed_clone = ctrl_pressed.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key_code() == 17 {
                ctrl_pressed_clone.set(false);
            }
        })
    };

    html! {
      <div
        class="w-[98vw] h-[98vh] m-h-[98vh] [content-visibility:auto] rounded-xl mx-4 my-2 p-4 flex flex-col gap-y-2 bg-zinc-800 cursor-text overflow-y-auto scroll-smooth"
        onclick={click_on_terminal_box}
      >
        { for (*logs).iter().map(|log_entry| html! { <div class="whitespace-pre-wrap">{">> "} {log_entry} </div>})}
         if *show_commands {
            <div class="flex flex-wrap gap-x-20">
                <span class="text-green-500">{"clear"}</span>
                <span class="text-green-500">{"cd"}</span>
                <span class="text-green-500">{"exit"}</span>
                <span class="text-green-500">{"clear"}</span>
                <span class="text-green-500">{"cd"}</span>
                <span class="text-green-500">{"exit"}</span>
                <span class="text-green-500">{"clear"}</span>
                <span class="text-green-500">{"cd"}</span>
                <span class="text-green-500">{"exit"}</span>
                <span class="text-green-500">{"clear"}</span>
                <span class="text-green-500">{"cd"}</span>
                <span class="text-green-500">{"exit"}</span>
                <span class="text-green-500">{"clear"}</span>
                <span class="text-green-500">{"cd"}</span>
                <span class="text-green-500">{"exit"}</span>
                <span class="text-green-500">{"clear"}</span>
                <span class="text-green-500">{"cd"}</span>
                <span class="text-green-500">{"exit"}</span>
                <span class="text-green-500">{"clear"}</span>
                <span class="text-green-500">{"cd"}</span>
                <span class="text-green-500">{"exit"}</span>
            </div>
        }
        <div class="flex flex-row w-full">
          <span class="w-1/12">{"guess@DA2Mdev:-$"}</span>
          <input
            type="text"
            class="inline-block outline-none w-11/12"
            id="terminal_input"
            autofocus=true ref={input_terminal}
            {onkeydown}
            {onkeypress}
            {onkeyup}
          />
        </div>
      </div>
    }
}
