#![forbid(unsafe_code)]

mod components;
mod hooks;
mod state;
mod util;

use components::app::Root;

/// Start the frontend Yew application.
fn main() {
    yew::Renderer::<Root>::new().render();
}
