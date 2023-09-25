use frontend_common::FrontendCommands;
use yewdux::prelude::*;

/// The frontend application state.
#[derive(Default, Clone, PartialEq, Eq, Store, FrontendCommands)]
pub struct State;
