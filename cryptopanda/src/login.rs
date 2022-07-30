use dioxus::events::*;
use dioxus::prelude::*;
use dioxus::router::{Link, Route, Router};

use crate::G_LOGINED;



pub fn Login(cx: Scope) -> Element {
    let is_logined = use_atom_ref(&cx, G_LOGINED);
    cx.render(rsx! {
        div {
            h1 { "Login" }
            form {
                onsubmit: move |ev| {
                    *is_logined.write()=true;
                    println!("Submitted {:?}", ev.values);
                }
                    ,
                oninput: move |ev| println!("Input {:?}", ev.values),
                h2 {"enter mnemonics to login"}  input { r#type: "text", name: "mnemonics", value:"enter your mnemonics" }
                button { r#type: "submit", value: "Submit", "Submit the form" }
            }

        }
    })
}
