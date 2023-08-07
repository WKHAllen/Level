use yew::prelude::*;

/// The size of an error message.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ErrorSize {
    /// A very small message.
    Smaller,
    /// A small message.
    Small,
    /// A medium sized message.
    #[default]
    Medium,
    /// A large message.
    Large,
    /// A very large message.
    Larger,
}

impl ErrorSize {
    pub fn size_name(&self) -> &'static str {
        match *self {
            Self::Smaller => "smaller",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Larger => "larger",
        }
    }
}

/// Error properties.
#[derive(Properties, PartialEq, Clone)]
pub struct ErrorProps {
    /// The error message.
    #[prop_or_default]
    pub message: Option<AttrValue>,
    /// The size of the error message.
    #[prop_or_default]
    pub size: ErrorSize,
}

/// An error element.
#[function_component]
pub fn Error(props: &ErrorProps) -> Html {
    let ErrorProps { message, size } = props.clone();

    let size_class = format!("base-text-{}", size.size_name());

    html! {
        <span class={classes!("base-error", size_class)}>{message.unwrap_or_default()}</span>
    }
}
