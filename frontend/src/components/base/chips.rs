use super::util::*;
use super::*;
use yew::prelude::*;

/// Chips properties.
#[derive(Properties, PartialEq, Clone)]
pub struct ChipsProps {
    /// The state of the currently selected chips.
    pub current_chips_state: UseStateHandle<Vec<String>>,
    /// The state of the chip being typed.
    pub next_chip_state: UseStateHandle<String>,
    /// The list of chip options.
    pub options: Vec<String>,
    /// The chips input label.
    #[prop_or_default]
    pub label: String,
    /// Chips input placeholder text.
    #[prop_or_default]
    pub placeholder: String,
    /// The maximum number of characters allowed in the chip input.
    #[prop_or(524288)]
    pub max_length: usize,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<String>,
    /// Whether the chip input is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A chip selection component.
#[function_component]
pub fn Chips(props: &ChipsProps) -> Html {
    let ChipsProps {
        current_chips_state,
        next_chip_state,
        options,
        label,
        placeholder,
        max_length,
        error,
        disabled,
    } = props.clone();

    let next_chip = (*next_chip_state).clone();
    let id_state = use_state(new_id);
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);
    let oninput = {
        let oninput_next_chip_state = next_chip_state.clone();
        move |event: InputEvent| {
            let new_next_chip = input_event_value(event);
            oninput_next_chip_state.set(new_next_chip);
        }
    };
    let onfocusin = {
        let dropdown_open_focusin = dropdown_open.clone();
        move |_| {
            dropdown_open_focusin.set(true);
        }
    };
    let onfocusout = {
        let dropdown_open_focusout = dropdown_open.clone();
        move |_| {
            dropdown_open_focusout.set(false);
        }
    };
    let onkeydown = {
        let first_option = options.first().map(|option| option.to_owned());
        let onkeydown_current_chips = current_chips_state.clone();
        let onkeydown_next_chip = next_chip_state.clone();
        move |event: KeyboardEvent| {
            if event.key_code() == 13 {
                if let Some(ref option) = first_option {
                    let mut chips = (*onkeydown_current_chips).clone();
                    chips.push(option.to_owned());
                    onkeydown_current_chips.set(chips);
                    onkeydown_next_chip.set(String::new());
                }
            }
        }
    };

    let chip_list = (*current_chips_state)
        .iter()
        .enumerate()
        .map(|(index, this_chip)| {
            let local_chips_state = current_chips_state.clone();

            let on_click = move |_| {
                let mut current_chips_without_this = (*local_chips_state).clone();
                current_chips_without_this.remove(index);
                local_chips_state.set(current_chips_without_this);
            };

            html! {
                <div class="base-chips-chip">
                    <span class="base-chips-chip-label">{this_chip}</span>
                    <IconButton
                        name="xmark-solid"
                        size={IconButtonSize::Small}
                        {disabled}
                        {on_click}
                        class="base-chips-chip-remove"
                    />
                </div>
            }
        })
        .collect::<Html>();

    let conditional_chip_list = if (*current_chips_state).is_empty() {
        html! {}
    } else {
        html! {
            <div class="base-chips-chip-list">
                {chip_list}
            </div>
        }
    };

    let chip_options = options
        .iter()
        .map(|this_option| {
            let this_option = this_option.clone();
            let this_option_html = this_option.clone();
            let option_current_chips_state = current_chips_state.clone();
            let option_next_chip_state = next_chip_state.clone();
            let option_onmousedown = move |_| {
                let mut option_chips = (*option_current_chips_state).clone();
                option_chips.push(this_option.clone());
                option_current_chips_state.set(option_chips);
                option_next_chip_state.set(String::new());
            };

            html! {
                <div class={classes!("base-chips-option")} onmousedown={option_onmousedown}>
                    {this_option_html}
                </div>
            }
        })
        .collect::<Html>();

    let conditional_chip_options = if options.is_empty() {
        html! {}
    } else {
        html! {
            <div class="base-chips-options-dropdown">
                <div class="base-chips-options-popup">
                    {chip_options}
                </div>
            </div>
        }
    };

    html! {
        <div class={classes!("base-chips-container", disabled.then_some("base-chips-container-disabled"), (*dropdown_open).then_some("base-chips-container-open"), error.as_ref().map(|_| "base-chips-container-invalid"))}>
            <label for={id.clone()} class="base-chips-label">{label}</label>
            <div class="base-chips">
                <div class="base-chips-inner">
                    {conditional_chip_list}
                    <input
                        type="text"
                        value={next_chip}
                        {id}
                        {oninput}
                        {onfocusin}
                        {onfocusout}
                        {onkeydown}
                        {placeholder}
                        {disabled}
                        maxlength={max_length.to_string()}
                        class="base-chips-input"
                    />
                </div>
                {conditional_chip_options}
            </div>
            <Error message={error} size={ErrorSize::Small} />
        </div>
    }
}
