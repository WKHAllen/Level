use crate::hooks::*;
use commands::FrontendCommands;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// A handle to the demo mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UseDemoHandle {
    /// Demo mode is still resolving.
    Unresolved,
    /// Demo mode is resolved to a value.
    Resolved(bool),
}

#[allow(clippy::from_over_into)]
impl Into<Option<bool>> for UseDemoHandle {
    fn into(self) -> Option<bool> {
        match self {
            Self::Unresolved => None,
            Self::Resolved(value) => Some(value),
        }
    }
}

/// Checks whether the app is in demo mode.
#[hook]
pub fn use_demo() -> UseDemoHandle {
    let (backend, _) = use_backend();

    let demo_state = use_state(|| UseDemoHandle::Unresolved);

    use_effect_once({
        let demo_state = demo_state.clone();
        move || {
            spawn_local(async move {
                let demo_mode = backend.demo_mode().await;
                demo_state.set(UseDemoHandle::Resolved(demo_mode));
            });
            || ()
        }
    });

    *demo_state
}
