use crate::components::views::*;
use gloo_storage::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

/// The session storage key for storing the app view.
pub const VIEW_STORAGE_KEY: &str = "APP_VIEW";

/// A view in the application.
#[derive(Default, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum View {
    /// Home view.
    #[default]
    Home,
    /// Open a save.
    Open,
    /// Create a new save.
    Create,
    /// A save view.
    Save,
    /// Settings view.
    Settings,
    /// Report view.
    Report,
    /// Report template view.
    ReportTemplate,
    /// Global search view.
    Search,
}

impl View {
    /// Renders the view as HTML.
    pub fn html_view(&self) -> Html {
        match self {
            Self::Home => html! { <Home /> },
            Self::Open => html! { <Open /> },
            Self::Create => html! { <Create /> },
            Self::Save => html! { <Save /> },
            Self::Settings => html! { <Settings /> },
            Self::Report => html! { <Report /> },
            Self::ReportTemplate => html! { <ReportTemplate /> },
            Self::Search => html! { <Search /> },
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for View {
    fn into(self) -> Html {
        self.html_view()
    }
}

impl Store for View {
    fn new() -> Self {
        if let Ok(view) = SessionStorage::get::<Self>(VIEW_STORAGE_KEY) {
            view
        } else {
            SessionStorage::set(VIEW_STORAGE_KEY, Self::default())
                .expect_throw("session storage failed to set view");

            Self::default()
        }
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}
