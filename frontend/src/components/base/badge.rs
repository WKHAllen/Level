use super::util::*;
use yew::prelude::*;

/// The style of a badge.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BadgeStyle {
    /// Primary style.
    #[default]
    Primary,
    /// Secondary style.
    Secondary,
    /// Danger style.
    Danger,
}

impl BadgeStyle {
    /// Gets the name of the badge style.
    pub fn style_name(&self) -> &'static str {
        match *self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Danger => "danger",
        }
    }
}

/// Badge properties.
#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps<N: Number> {
    /// The badge value.
    pub value: N,
    /// The badge style.
    #[prop_or_default]
    pub style: BadgeStyle,
}

/// A badge component.
#[function_component]
pub fn Badge<N: Number = usize>(props: &BadgeProps<N>) -> Html {
    let BadgeProps { value, style } = props.clone();

    let style_class = format!("base-badge-{}", style.style_name());

    html! {
        <div class="base-badge-container">
            <div class={classes!("base-badge", style_class)}>
                <div class="base-badge-text">{value.to_string()}</div>
            </div>
        </div>
    }
}
