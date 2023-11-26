use crate::components::subviews::*;
use yew::prelude::*;

/// An application subview.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Subview {
    /// Account creation subview.
    CreateAccount,
    /// Account editing subview.
    EditAccount,
    /// Category editing subview.
    EditCategories,
    /// Subcategory editing subview.
    EditSubcategories,
    /// Tag editing subview.
    EditTags,
    /// Institution editing subview.
    EditInstitutions,
}

impl Subview {
    /// Renders the subview as HTML.
    pub fn html_subview(&self) -> Html {
        match self {
            Self::CreateAccount => html! { <CreateAccount /> },
            Self::EditAccount => html! { <EditAccount /> },
            Self::EditCategories => html! { <EditCategories /> },
            Self::EditSubcategories => html! { <EditSubcategories /> },
            Self::EditTags => html! { <EditTags /> },
            Self::EditInstitutions => html! { <EditInstitutions /> },
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for Subview {
    fn into(self) -> Html {
        self.html_subview()
    }
}
