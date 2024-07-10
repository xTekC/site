#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;", // Added container styles
            h1 { "TekC" }
            img { src: "logo_192.png" }
            p {
                "Hello"
            }
            Link {
                to: Route::Blog {},
                style: "display: flex; justify-content: center; align-items: center; color: orange;",
                "Blog"
            }
        }
    }
}
