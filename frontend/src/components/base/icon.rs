use yew::prelude::*;

/// Icon properties.
#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    /// Icon name.
    pub name: String,
    /// Whether the icon is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Classes to apply to the icon.
    #[prop_or_default]
    pub class: Classes,
}

/// An icon component.
#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    let IconProps {
        name,
        disabled,
        mut class,
    } = props.clone();

    let svg_path = format!("assets/svg/{}.svg", name);
    class.push("base-icon");

    if disabled {
        class.push("base-icon-disabled");
    }

    html! {
        <img src={svg_path} {class} />
    }
}
