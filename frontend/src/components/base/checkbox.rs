use super::util::*;
use yew::prelude::*;

/// Checkbox properties.
#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    /// The checkbox state.
    pub state: UseStateHandle<bool>,
    /// The checkbox label.
    #[prop_or_default]
    pub label: String,
    /// Whether the checkbox is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A checkbox component.
#[function_component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
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
        <div class="base-checkbox-container">
            <label class={classes!("base-checkbox", disabled.then_some("base-checkbox-disabled"))}>
                <span class="base-checkbox-label">{label}</span>
                <input
                    type="checkbox"
                    {checked}
                    {onclick}
                    {disabled}
                    class="base-checkbox-input"
                />
                <span class="base-checkmark">
                    <img src="assets/svg/check-solid.svg" class="base-checkmark-icon" />
                </span>
            </label>
        </div>
    }
}
