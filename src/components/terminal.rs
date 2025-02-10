use wasm_bindgen::UnwrapThrowExt;
use web_sys::{console, window};
use yew::{function_component, html, Html};

#[function_component]
pub fn Terminal() -> Html {
    // let document = window()
    //     .expect_throw("windows is undefined")
    //     .document()
    //     .expect_throw("document is undefined");
    // console::log_1(&"test".to_string());
    // let terminal_element = document
    //     .get_element_by_id(&"terminal")
    //     .expect_throw("terminal is undefined");
    // console::log_1(&terminal_element);

    html! {
      <div class="my-5 flex flex-col gap-y-2 bg-zinc-800 cursor-text">
        <div>{"log"}</div>
        <div class="flex flex-row w-full">
          <span class="w-1/10">{"# [guess@DA2Mdev]:"}</span>
          <input type="text" class="inline-block outline-none w-9/10" id="terminal" autofocus=true />
        </div>
      </div>
    }
}
