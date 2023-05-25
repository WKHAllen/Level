use yew::prelude::*;

/// The size of a spinner.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SpinnerSize {
    /// A small spinner.
    Small,
    /// A medium size spinner.
    #[default]
    Medium,
    /// A large spinner.
    Large,
    /// A spinner that grows to the size of the container.
    Max,
}

impl SpinnerSize {
    /// Gets the name of the spinner size.
    pub fn size_name(&self) -> &'static str {
        match *self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Max => "max",
        }
    }
}

/// Spinner properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SpinnerProps {
    /// The size of the spinner.
    #[prop_or_default]
    pub size: SpinnerSize,
    /// Whether to center the spinner.
    #[prop_or(true)]
    pub center: bool,
}

/// A spinner component.
#[function_component]
pub fn Spinner(props: &SpinnerProps) -> Html {
    let SpinnerProps { size, center } = props.clone();

    let size_class = format!("base-spinner-{}", size.size_name());

    html! {
        <div class={classes!("base-spinner-container", center.then_some("base-spinner-center"))}>
            <svg class={classes!("base-spinner", size_class)} viewBox="0 0 50 50">
                <circle
                    class="path"
                    cx="25"
                    cy="25"
                    r="20"
                    fill="none"
                    stroke-width="5"
                ></circle>
            </svg>
        </div>
    }
}
