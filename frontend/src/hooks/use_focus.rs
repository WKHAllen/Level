use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

/// A handle to an element focusing operation.
#[derive(Debug, Clone)]
pub struct UseFocusHandle {
    /// The node to focus.
    node: NodeRef,
}

#[allow(dead_code)]
impl UseFocusHandle {
    /// Returns a clone of the inner node ref.
    pub fn node_ref(&self) -> NodeRef {
        self.node.clone()
    }

    /// Focuses the element.
    pub fn focus(&self) {
        if let Some(node) = self.node.get() {
            node.dyn_ref::<HtmlElement>().unwrap().focus().unwrap();
        }
    }

    /// Waits 100 milliseconds, and then focuses the element.
    pub fn focus_late(&self) {
        self.focus_after(100);
    }

    /// Focuses the element after a number of milliseconds.
    pub fn focus_after(&self, ms: u32) {
        let node = self.node.get();

        Timeout::new(ms, || {
            if let Some(node) = node {
                node.dyn_ref::<HtmlElement>().unwrap().focus().unwrap();
            }
        })
        .forget();
    }
}

/// Focus an element.
#[hook]
pub fn use_focus() -> UseFocusHandle {
    let node = use_node_ref();

    UseFocusHandle { node }
}
