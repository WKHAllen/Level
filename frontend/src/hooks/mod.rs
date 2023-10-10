mod use_async;
mod use_demo;
mod use_file_select;
mod use_id;
mod use_popup;
mod use_theme;
mod use_view;

pub use use_async::*;
pub use use_demo::*;
pub use use_file_select::*;
pub use use_id::*;
pub use use_popup::*;
pub use use_theme::*;
pub use use_view::*;

use crate::backend::BackendHandle;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

/// Generates a new hook for a given store value.
macro_rules! store_hook {
    ( $name:ident, $ty:ty, $doc:literal ) => {
        #[doc = $doc]
        #[hook]
        pub fn $name() -> (Rc<$ty>, Dispatch<$ty>) {
            use_store::<$ty>()
        }
    };
}

store_hook!(
    use_backend,
    BackendHandle,
    "Gets a handle to the backend of the application."
);
