use crate::hooks::*;
use yew::prelude::*;

/// Tooltip properties.
#[derive(Properties, PartialEq, Clone)]
pub struct TooltipProps {
    /// The tooltip text.
    pub text: AttrValue,
    /// Whether the tooltip is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    #[prop_or_default]
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

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-tooltip", disabled.then_some("base-tooltip-disabled"), class)}>
            {children}
            <div class="base-tooltip-container">
                <div class="base-tooltip-popup-container">
                    <div ref={popup_node} class="base-tooltip-popup">
                        <span class="base-tooltip-text">{text}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
