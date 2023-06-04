#![allow(dead_code)]

use js_sys::Math;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlElement, HtmlInputElement, HtmlTextAreaElement, InputEvent, MouseEvent};
use yew::prelude::*;

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

/// Gets the value of a content-editable element from an event.
pub fn content_editable_event_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlElement = event_target.dyn_into().unwrap_throw();
    target.inner_text()
}

/// Gets the value of a checkbox from a mouse click event.
pub fn checkbox_checked(e: MouseEvent) -> bool {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.checked()
}

/// Focuses an element in the DOM.
pub fn focus_element(node: &NodeRef) {
    if let Some(node) = node.get() {
        node.dyn_ref::<HtmlElement>().unwrap().focus().unwrap();
    }
}

/// Selects the content of an element in the DOM.
pub fn select_element_content(node: &NodeRef) {
    if let Some(node) = node.get() {
        let window = web_sys::window().unwrap();

        let range = web_sys::Range::new().unwrap();
        range.select_node_contents(&node).unwrap();

        let selection = window.get_selection().unwrap().unwrap();
        selection.remove_all_ranges().unwrap();
        selection.add_range(&range).unwrap();
    }
}

/// Sets the cursor position to the end within an element in the DOM.
pub fn go_to_end(node: &NodeRef) {
    if let Some(node) = node.get() {
        let window = web_sys::window().unwrap();

        let selection = window.get_selection().unwrap().unwrap();
        selection.set_position_with_offset(Some(&node), 1).unwrap();
    }
}

/// Clears all selections.
pub fn clear_selections() {
    let window = web_sys::window().unwrap();

    let selection = window.get_selection().unwrap().unwrap();
    selection.remove_all_ranges().unwrap();
}

/// Sets the inner text of an element in the DOM.
pub fn set_inner_text(node: &NodeRef, text: &str) {
    if let Some(node) = node.get() {
        node.set_text_content(Some(text));
    }
}

/// Generates a random ID for an element.
pub fn new_id() -> String {
    let value = Math::random().to_bits();
    let hex_value = format!("{value:x}");
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

/// A trait for numeric values.
pub trait Number:
    PartialEq
    + PartialOrd
    + FromStr
    + ToString
    + Default
    + Clone
    + Copy
    + Display
    + Debug
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
    const NUMBER_MIN: Self;
    const NUMBER_MAX: Self;
    const NUMBER_STEP: Self;
    const DECIMAL: bool;

    fn as_f64(self) -> f64;
}

/// Implements the `Number` trait for integer primitives.
macro_rules! impl_number_int {
    ( $($ty:ty),* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1 as Self;
                const DECIMAL: bool = false;

                fn as_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

/// Implements the `Number` trait for floating point primitives.
macro_rules! impl_number_float {
    ( $($ty:ty),* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1 as Self;
                const DECIMAL: bool = true;

                fn as_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

impl_number_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl_number_float!(f32, f64);
