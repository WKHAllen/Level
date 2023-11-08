use crate::util::*;
use yew::prelude::*;

/// Checkbox properties.
#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    /// The checkbox state.
    pub state: UseStateHandle<bool>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// The checkbox label.
    #[prop_or_default]
    pub label: AttrValue,
    /// Whether the checkbox is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The checkbox input node ref.
    #[prop_or_default]
    pub node: NodeRef,
}

/// A checkbox component.
#[function_component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        state,
        on_change,
        label,
        disabled,
        node,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(**new_state));

    let checked = *state;
    let onclick = move |event: MouseEvent| {
        let new_value = checkbox_checked(event);
        state.set(new_value);
    };

    html! {
        <div class="base-checkbox-container">
            <label class={classes!("base-checkbox", disabled.then_some("base-checkbox-disabled"))}>
                <span class="base-checkbox-label">{label}</span>
                <input
                    type="checkbox"
                    {checked}
                    {onclick}
                    {disabled}
                    class="base-checkbox-input"
                    ref={node}
                />
                <span class="base-checkmark">
                    <img src="assets/svg/check-solid.svg" class="base-checkmark-icon" />
                </span>
            </label>
        </div>
    }
}
