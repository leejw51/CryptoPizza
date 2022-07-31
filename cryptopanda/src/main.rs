mod location;
mod login;
mod order;
mod program;
mod review;
mod vieworder;
use dioxus::prelude::*;
use dioxus::router::{Link, Route, Router};
use program::G_PROGRAM;
fn main() {
    G_PROGRAM.lock().unwrap().initialize();
    dioxus::desktop::launch(app);
}

pub static G_LOGINED: AtomRef<bool> = |_| false;

fn app(cx: Scope) -> Element {
    let is_logined = use_atom_ref(&cx, G_LOGINED);

    cx.render(rsx! (
        style { [include_str!("../assets/ibc.css")] }
        Router {


        div {
                div {
                    class: "calculator-display",
                    h1 {"IBC pizza"}
                }


                if !*is_logined.read() {
                    rsx! (


                        div {
                            class: "pizzamenu",
                            img { src: "assets/ibc.png", }
                            img { src: "assets/pizza.jpeg", }

                            Link { to: "/login",
                                button {
                                    class: "calculator-key",
                                    h2 { "Login" }
                                }
                            }
                        }
                    )
                } else {
                    rsx! (

                        div {
                        class: "pizzamenu",
                        img { src: "assets/ibc.png", }
                        img { src: "assets/pizza.jpeg", }

                            button {
                                class: "calculator-key",
                                onclick: move |_| {
                                    *is_logined.write()=false;
                                },
                                h2 { "Logout" }
                            }


                        Link { to: "/order",
                            button {
                                class: "calculator-key",
                                h2 { "Order" }
                            }
                        }


                        Link { to: "/vieworder",
                            button {
                                class: "calculator-key",
                                h2 { "ViewOrder"}
                            }
                        }

                        Link { to: "/review",
                            button {
                                class: "calculator-key",
                                h2 { "Review" }
                            }
                        }

                        Link { to: "/location",
                            button {
                                class: "calculator-key",
                                h2 { "Location" }
                            }
                        }
                    }

                    )
                }



        }

        Route { to : "/login",  crate::login::login{} }
        Route { to: "/order", crate::order::order{} }
        Route { to: "/review", crate::review::review{} }
        Route { to:"/vieworder", crate::vieworder::view_order{} }
        Route { to:"/location", crate::location::location{} }
        }

    )) // end of render
}

/*

fn main() {
    println!("Crypto Panda Dapp");
    println!("1. connect wallet");
    println!("2. register address");
    println!("3. order food");
    println!("4. deliver food");
    println!("5. view delivers");
    println!("6. evaluate food");
    println!("7. view orders");
}*/
