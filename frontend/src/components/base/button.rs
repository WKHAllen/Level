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
    /// Gets the name of the button style.
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
    pub text: AttrValue,
    /// The button style.
    #[prop_or_default]
    pub style: ButtonStyle,
    /// Whether the button is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The button click callback.
    #[prop_or_default]
    pub on_click: Callback<()>,
    /// The button node ref.
    #[prop_or_default]
    pub node: NodeRef,
}

/// A button component.
#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps {
        text,
        style,
        disabled,
        on_click,
        node,
    } = props.clone();

    let style_class = format!("base-button-{}", style.style_name());
    let onclick = move |_| {
        if !disabled {
            on_click.emit(());
        }
    };

    html! {
        <button
            type="button"
            {onclick}
            {disabled}
            class={classes!("base-button", style_class)}
            ref={node}
        >
            {text}
        </button>
    }
}
