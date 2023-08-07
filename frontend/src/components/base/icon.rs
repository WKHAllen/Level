use yew::prelude::*;

/// The size of an icon.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum IconSize {
    /// A small icon.
    Small,
    /// A medium icon.
    #[default]
    Medium,
    /// A large icon.
    Large,
}

impl IconSize {
    /// Gets the name of the icon size.
    pub fn size_name(&self) -> &'static str {
        match *self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Icon properties.
#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    /// Icon name.
    pub name: AttrValue,
    /// The size of the icon.
    #[prop_or_default]
    pub size: IconSize,
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
        size,
        disabled,
        mut class,
    } = props.clone();

    let size_class = format!("base-icon-{}", size.size_name());
    let svg_path = format!("assets/svg/{name}.svg");
    class.push("base-icon");
    class.push(size_class);

    if disabled {
        class.push("base-icon-disabled");
    }

    html! {
        <img src={svg_path} {class} />
    }
}
