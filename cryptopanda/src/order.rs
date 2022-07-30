use dioxus::events::*;
use dioxus::prelude::*;
use dioxus::router::{Link, Route, Router};

pub fn Order(cx: Scope) -> Element {

    cx.render(rsx! {
        div {
            h1 { "Order" }

            h2 { "1. super supreme pizza 20000 PZD" }
            h2 { "2. vegetable 21000 PZD" }
            h2 { "3. pepperoni 18000 PZD" }
        }
    })
}
