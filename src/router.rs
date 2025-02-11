use crate::components::mode::Mode;
use crate::pages::{home_standard::HomeStandard, home_terminal::HomeTerminal};
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Mode,
    #[at("/standard")]
    Standard,
    #[at("/terminal")]
    Terminal,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Mode => html! {<Mode />},
        Route::Standard => html! {<HomeStandard />},
        Route::Terminal => html! {<HomeTerminal />},
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}
