use frontend_common::FrontendCommands;
use yewdux::prelude::*;

/// A handle to the backend.
#[derive(Default, PartialEq, Store, FrontendCommands)]
pub struct BackendHandle;
