use crate::util::*;
use yew::prelude::*;

/// Switch properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps {
    /// The switch state.
    pub state: UseStateHandle<bool>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// The switch label.
    #[prop_or_default]
    pub label: AttrValue,
    /// Whether the switch is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A switch component.
#[function_component]
pub fn Switch(props: &SwitchProps) -> Html {
    let SwitchProps {
        state,
        on_change,
        label,
        disabled,
    } = props.clone();

    use_effect_with_deps(move |new_state| on_change.emit(**new_state), state.clone());

    let checked = *state;
    let onclick = move |event: MouseEvent| {
        let new_value = checkbox_checked(event);
        state.set(new_value);
    };

    html! {
        <div class="base-switch-container">
            <label class={classes!("base-switch", disabled.then_some("base-switch-disabled"))}>
                <span class="base-switch-label">{label}</span>
                <input
                    type="checkbox"
                    {checked}
                    {onclick}
                    {disabled}
                    class="base-switch-input"
                />
                <span class="base-switch-toggle"></span>
            </label>
        </div>
    }
}
