use yew::prelude::*;

/// Radio button properties.
#[derive(Properties, PartialEq, Clone)]
pub struct RadioButtonProps {
    /// Whether the radio button is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    pub children: Children,
}

/// A radio button component.
#[function_component]
pub fn RadioButton(props: &RadioButtonProps) -> Html {
    let RadioButtonProps { children, .. } = props.clone();

    html! {
        <>
            {children}
        </>
    }
}
