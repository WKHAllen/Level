use super::util::*;
use yew::prelude::*;

/// Switch properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps {
    /// The switch state.
    pub state: UseStateHandle<bool>,
    /// The switch label.
    #[prop_or_default]
    pub label: String,
    /// Whether the switch is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A switch component.
#[function_component]
pub fn Switch(props: &SwitchProps) -> Html {
    let SwitchProps {
        state,
        label,
        disabled,
    } = props.clone();

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
