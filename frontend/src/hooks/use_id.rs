use crate::util::new_id;
use yew::prelude::*;

/// Generates a random ID which persists for the full lifetime of the element.
#[hook]
pub fn use_id() -> UseStateHandle<String> {
    use_state(new_id)
}
