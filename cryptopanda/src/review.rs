use dioxus::events::*;
use dioxus::prelude::*;
use dioxus::router::{Link, Route, Router};
use crate::G_LOGINED;

pub fn Review(cx: Scope) -> Element {
    let is_logined = use_atom_ref(&cx, G_LOGINED);
    cx.render(rsx! {
        if !*is_logined.read() {
            rsx!(div{})
        }
        else {
            rsx!(
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
            )

        }
        
    })
}
