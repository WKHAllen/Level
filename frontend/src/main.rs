#![forbid(unsafe_code)]

mod backend;
mod components;
mod hooks;
mod util;
mod view;

use components::App;

/// Start the frontend Yew application.
fn main() {
    yew::Renderer::<App>::new().render();
}
