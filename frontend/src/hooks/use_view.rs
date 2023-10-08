use crate::view::*;
use gloo_storage::*;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

/// A handle to the application's current view.
#[derive(Clone)]
pub struct UseViewHandle {
    /// The inner value.
    value: Rc<View>,
    /// The dispatcher for the inner value.
    dispatch: Dispatch<View>,
}

impl UseViewHandle {
    /// Sets the new view state.
    pub fn set(&self, value: View) {
        self.dispatch.set(value);

        SessionStorage::set(VIEW_STORAGE_KEY, value)
            .expect_throw("session storage failed to set view");
    }
}

impl Deref for UseViewHandle {
    type Target = View;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Gets a handle to the current application view.
#[hook]
pub fn use_view() -> UseViewHandle {
    let (view, dispatch_view) = use_store::<View>();

    UseViewHandle {
        value: view,
        dispatch: dispatch_view,
    }
}
