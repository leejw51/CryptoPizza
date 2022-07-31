use crate::G_LOGINED;
use dioxus::prelude::*;
pub fn view_order(cx: Scope) -> Element {
    let is_logined = use_atom_ref(&cx, G_LOGINED);
    cx.render(rsx! {
        if !*is_logined.read() {
            rsx!(div{})
        }
        else {
            rsx!(
                div {
                    h1 { "ViewOrder" }

                    h2 { "1. super supreme pizza 2 EA" }
                    h2 { "2. pepperoni pizza 1 EA" }
                }
            )
        }

    })
}
