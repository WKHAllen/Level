use yew::prelude::*;

/// An application subview.
#[derive(Debug, Clone, PartialEq)]
pub struct Subview(Html);

impl Subview {
    /// Renders the subview as HTML.
    pub fn html_subview(&self) -> Html {
        self.0.clone()
    }
}

impl From<Html> for Subview {
    fn from(value: Html) -> Self {
        Self(value)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for Subview {
    fn into(self) -> Html {
        self.html_subview()
    }
}
