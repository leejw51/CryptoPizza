use crate::G_LOGINED;
use dioxus::prelude::*;
pub fn order(cx: Scope) -> Element {
    let is_logined = use_atom_ref(&cx, G_LOGINED);
    cx.render(rsx! {
        if !*is_logined.read() {
            rsx!(div{})
        }
        else {
            rsx!(
                div {
                    h1 { "Order" }

                    h2 { "1. super supreme pizza 20000 PZD" }
                    h2 { "2. vegetable 21000 PZD" }
                    h2 { "3. pepperoni 18000 PZD" }
                }
            )
        }

    })
}
