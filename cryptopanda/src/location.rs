use crate::G_LOGINED;
use dioxus::prelude::*;
pub fn location(cx: Scope) -> Element {
    let is_logined = use_atom_ref(&cx, G_LOGINED);
    cx.render(rsx! {
        if !*is_logined.read() {
            rsx!(div{})
        }
        else {
            rsx!(
            div {
                h1 { "Where" }
                form {
                    onsubmit: move |ev| println!("Submitted {:?}", ev.values),
                    oninput: move |ev| println!("Input {:?}", ev.values),
                    h2 {"delivery to "}  input { r#type: "text", name: "location", value:"12345678 IFC mall" }
                    button { r#type: "submit", value: "Submit", "Submit the form" }
                }
           }
        )
        }  // end of else
    } // end of rsx
    )
}
