#![forbid(unsafe_code)]

mod components;
mod hooks;
mod state;

use components::App;

/// Start the frontend Yew application.
fn main() {
    yew::Renderer::<App>::new().render();
}
