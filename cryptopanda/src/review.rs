use dioxus::events::*;
use dioxus::prelude::*;
use dioxus::router::{Link, Route, Router};


pub fn Review(cx: Scope) -> Element {

    cx.render(rsx! {
        div {
            h1 { "Review" }

            button {
                h2 { "1. good"}
            }

            button {
                h2 { "2. normal"}
            }

            button {
                h2 { "3. not good"}
            }
        }
    })
}
