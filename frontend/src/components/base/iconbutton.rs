use super::*;
use yew::prelude::*;

/// The size of an icon button.
pub type IconButtonSize = IconSize;

/// Icon button properties.
#[derive(Properties, PartialEq, Clone)]
pub struct IconButtonProps {
    /// Icon name.
    pub name: String,
    /// The size of the icon button.
    #[prop_or_default]
    pub size: IconButtonSize,
    /// Whether the icon button is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The icon button click callback.
    #[prop_or(Callback::from(|_| ()))]
    pub on_click: Callback<()>,
    /// Classes to apply to the icon.
    #[prop_or_default]
    pub class: Classes,
}

/// An icon button component.
#[function_component]
pub fn IconButton(props: &IconButtonProps) -> Html {
    let IconButtonProps {
        name,
        size,
        disabled,
        on_click,
        mut class,
    } = props.clone();

    let size_class = format!("base-icon-button-{}", size.size_name());
    let svg_path = format!("assets/svg/{name}.svg");
    class.push("base-icon-button-icon");
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
            class={classes!("base-icon-button", size_class)}
        >
            <img src={svg_path} {class} />
        </button>
    }
}
