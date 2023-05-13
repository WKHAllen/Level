use yew::prelude::*;

/// The style of a button.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonStyle {
    /// Primary style.
    #[default]
    Primary,
    /// Secondary style.
    Secondary,
    /// Transparent style.
    Transparent,
    /// Danger style.
    Danger,
}

impl ButtonStyle {
    pub fn style_name(&self) -> &'static str {
        match *self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Transparent => "transparent",
            Self::Danger => "danger",
        }
    }
}

/// Button properties.
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    /// The text on the button.
    pub text: String,
    /// The button style.
    #[prop_or_default]
    pub style: ButtonStyle,
    /// Whether the button is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The button click callback.
    #[prop_or(Callback::from(|_| ()))]
    pub on_click: Callback<()>,
}

/// A button component.
#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps {
        text,
        style,
        disabled,
        on_click,
    } = props.clone();

    let style_class = format!("base-button-{}", style.style_name());
    let onclick = move |_| {
        on_click.emit(());
    };

    html! {
        <button
            type="button"
            {onclick}
            {disabled}
            class={classes!("base-button", style_class)}
        >
            {text}
        </button>
    }
}
