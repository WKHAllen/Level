use yew::prelude::*;

/// Icon button properties.
#[derive(Properties, PartialEq, Clone)]
pub struct IconButtonProps {
    /// Icon name.
    pub name: String,
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
        disabled,
        on_click,
        mut class,
    } = props.clone();

    let svg_path = format!("assets/svg/{}.svg", name);
    class.push("base-icon-button-icon");
    let onclick = move |_| {
        on_click.emit(());
    };

    html! {
        <button
            {onclick}
            {disabled}
            class="base-icon-button"
        >
            <img src={svg_path} {class} />
        </button>
    }
}
