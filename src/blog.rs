#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Blog() -> Element {
    rsx! {
        Link {
            to: Route::Home {},
            style: "display: flex; justify-content: center; align-items: center; color: orange;",
            "Home"
        }
    }
}
