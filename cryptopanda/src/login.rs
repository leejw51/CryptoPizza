use dioxus::prelude::*;

use crate::G_LOGINED;

pub fn login(cx: Scope) -> Element {
    let is_logined = use_atom_ref(&cx, G_LOGINED);
    cx.render(rsx! {
        if !*is_logined.read() {
            rsx! {
        div {
            h1 { "Login" }
            form {
                onsubmit: move |ev| {
                    *is_logined.write()=true;
                    println!("Submitted {:?}", ev.values);
                }
                    ,
                oninput: move |ev| println!("Input {:?}", ev.values),
                h2 {"enter mnemonics to login"}  input {class: "new-todo", r#type: "text", name: "mnemonics", value:"enter your mnemonics" }
                button { class:"new-todo", r#type: "submit", value: "Submit", "Submit the form" }
            }

        } // end of div
        } // end of rsx
    }
    else {
        rsx! { div {}}
    }
    }) // end of rsx
}
