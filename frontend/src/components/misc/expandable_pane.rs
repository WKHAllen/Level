use crate::components::base::*;
use yew::prelude::*;

/// Expandable pane properties.
#[derive(Properties, PartialEq, Clone)]
pub struct ExpandablePaneProps {
    /// The open/closed state of the pane.
    pub state: UseStateHandle<bool>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// The pane label.
    pub label: AttrValue,
    /// The classes applied to the pane label.
    #[prop_or_default]
    pub class: Classes,
    /// Child elements.
    #[prop_or_default]
    pub children: Children,
}

/// An expandable and collapsible pane.
#[function_component]
pub fn ExpandablePane(props: &ExpandablePaneProps) -> Html {
    let ExpandablePaneProps {
        state,
        on_change,
        label,
        class,
        children,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(**new_state));

    let open = *state;
    let onclick = move |_| state.set(!open);

    html! {
        <div class={classes!("expandable-pane", open.then_some("expandable-pane-open"))}>
            <div class={classes!("expandable-pane-label", class)} {onclick}>
                <Icon
                    name="angle-right-solid"
                    size={IconSize::Medium}
                    class="expandable-pane-icon"
                />
                <span>{label}</span>
            </div>
            <div class="expandable-pane-body">
                <div class="expandable-pane-body-inner">
                    {children}
                </div>
            </div>
        </div>
    }
}
