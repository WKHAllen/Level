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

    let hovering_state = use_state(|| false);

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    let on_hover_start = {
        let hovering_state = hovering_state.clone();
        move |_| {
            hovering_state.set(true);
        }
    };
    let on_hover_end = {
        let hovering_state = hovering_state.clone();
        move |_| {
            hovering_state.set(false);
        }
    };

    html! {
        <div class={classes!("base-tooltip", hovering_state.then_some("base-tooltip-open"), disabled.then_some("base-tooltip-disabled"))}>
            <div
                class={classes!("base-tooltip-content", class)}
                onmouseenter={on_hover_start}
                onmouseleave={on_hover_end}
            >
                {children}
            </div>
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
