use dioxus::events::*;
use dioxus::prelude::*;
use dioxus::router::{Link, Route, Router};

pub fn ViewOrder(cx: Scope) -> Element {

    cx.render(rsx! {
        div {
            h1 { "ViewOrder" }

            h2 { "1. super supreme pizza 2 EA" }
            h2 { "2. pepperoni pizza 1 EA" }
        }
    })
}
