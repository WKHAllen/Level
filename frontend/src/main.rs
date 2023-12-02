//! The level frontend.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod backend;
mod components;
mod hooks;
mod subview;
mod util;
mod validation;
mod view;

use components::App;

/// Start the frontend Yew application.
fn main() {
    yew::Renderer::<App>::new().render();
}
