use crate::components::views::*;
use yew::prelude::*;
use yewdux::prelude::*;

/// A view in the application.
#[derive(Default, PartialEq, Clone, Copy, Store)]
pub enum View {
    /// Home view.
    #[default]
    Home,
    /// Settings view.
    Settings,
    /// Open save view.
    Save,
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
            Self::Settings => html! { <Settings /> },
            Self::Save => html! { <Save /> },
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
