use dioxus::events::*;
use dioxus::prelude::*;
use dioxus::router::{Link, Route, Router};

pub fn Location(cx: Scope) -> Element {

    cx.render(rsx! {
        div {
            h1 { "Where" }
        

            form {
                onsubmit: move |ev| println!("Submitted {:?}", ev.values),
                oninput: move |ev| println!("Input {:?}", ev.values),
                h2 {"delivery to "}  input { r#type: "text", name: "location", value:"12345678 IFC mall" }
                button { r#type: "submit", value: "Submit", "Submit the form" }
            }
            
        }
    })
}
