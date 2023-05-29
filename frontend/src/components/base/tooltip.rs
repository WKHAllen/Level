use yew::prelude::*;

/// Tooltip properties.
#[derive(Properties, PartialEq, Clone)]
pub struct TooltipProps {
    /// The tooltip text.
    pub text: String,
    /// Whether the tooltip is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    pub children: Children,
    /// Class to apply to the tooltip container.
    #[prop_or_default]
    pub class: Classes,
}

/// A tooltip component.
#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let TooltipProps {
        text,
        disabled,
        children,
        class,
    } = props.clone();

    html! {
        <div class={classes!("base-tooltip-container", disabled.then_some("base-tooltip-container-disabled"), class)}>
            {children}
            <div class="base-tooltip">
                <div class="base-tooltip-text">{text}</div>
            </div>
        </div>
    }
}
