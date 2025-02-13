use crate::router::Route;
use web_sys::HtmlInputElement;
use yew::{hook, use_node_ref, use_state, Callback, KeyboardEvent, NodeRef, UseStateHandle};
use yew_router::{hooks::use_navigator, prelude::Navigator};

fn handle_clear_logs(logs: &UseStateHandle<Vec<String>>) {
    logs.set(Vec::<String>::new());
}

fn handle_route(navigation: &Navigator, route_segments: Vec<&str>) {
    let current_path_segment = route_segments.last().copied().unwrap_or("");
    const BACK_ROUTES: &[&str] = &["/view", "/", "..", "exit"];

    if BACK_ROUTES.contains(&current_path_segment) {
        navigation.back()
    }
    let cd_route: Vec<&str> = current_path_segment
        .split("/")
        .filter(|s| !s.is_empty())
        .collect();

    match cd_route.first() {
        Some(&"view") => {
            if cd_route.last().copied() == Some(&"standard") {
                navigation.push(&Route::Standard);
            }
        }
        _ => {}
    }
}

#[hook]
pub fn use_commands() -> (
    NodeRef,
    UseStateHandle<Vec<String>>,
    UseStateHandle<bool>,
    Callback<KeyboardEvent>,
) {
    let input_terminal = use_node_ref();
    let logs = use_state(|| Vec::<String>::new());
    let show_commands = use_state(|| false);
    let navigation = use_navigator().unwrap();

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
                    let input_value: String = input_node.value();
                    let value_splited: Vec<&str> = input_value
                        .as_str()
                        .split(" ")
                        .filter(|&s| !s.is_empty())
                        .collect();
                    match value_splited[0] {
                        "clear" => handle_clear_logs(&logs_clone),
                        "cd" => handle_route(&navigation_clone, value_splited),
                        "exit" => navigation.back(),
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

    (input_terminal, logs, show_commands, onkeypress)
}
