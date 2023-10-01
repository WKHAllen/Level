mod use_async;
mod use_file_select;
mod use_id;
mod use_popup;
mod use_theme;

pub use use_async::*;
pub use use_file_select::*;
pub use use_id::*;
pub use use_popup::*;
pub use use_theme::*;

use crate::backend::BackendHandle;
use crate::view::View;
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

store_hook!(
    use_view,
    View,
    "Gets the current view state of the application."
);
