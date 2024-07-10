// src/main.rs
#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
mod blog;
mod home;

use crate::blog::Blog;
use crate::home::Home;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    #[route("/blog/")]
    Blog,
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
