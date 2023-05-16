use js_sys::Math;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, HtmlTextAreaElement, InputEvent, MouseEvent};

/// Gets the value of an input element from an event.
pub fn input_event_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Gets the value of a textarea element from an event.
pub fn textarea_event_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Gets the value of a checkbox from a mouse click event.
pub fn checkbox_checked(e: MouseEvent) -> bool {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.checked()
}

/// Generates a random ID for an element.
pub fn new_id() -> String {
    let value = Math::random().to_bits();
    let hex_value = format!("{:x}", value);
    hex_value
}

/// Logs to the console.
#[allow(unused_macros)]
macro_rules! console_log {
    ( $($arg:tt)* ) => {{
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($arg)*)));
    }};
}

#[allow(unused_imports)]
pub(crate) use console_log;
