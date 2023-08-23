use super::*;
use crate::util::*;
use yew::prelude::*;
use yew_hooks::use_click_away;

/// Compares an option to a typed out value, returning a score indicating the
/// strength of the match, or `None` if the strings do not match.
fn option_match(option: &str, value: &str) -> Option<usize> {
    let option = option.to_lowercase();
    let value = value.to_lowercase();
    let mut score = 0;
    let mut indices_since_last_match = 0;
    let option_chars = option.chars();
    let mut value_chars = value.chars().peekable();
    let mut any_match = false;

    if option == value {
        return Some(0);
    }

    for option_char in option_chars {
        indices_since_last_match += 1;

        match value_chars.peek() {
            Some(value_char) => {
                if option_char == *value_char {
                    score += indices_since_last_match;
                    indices_since_last_match = 0;
                    value_chars.next();
                    any_match = true;
                }
            }
            None => break,
        }
    }

    if any_match && value_chars.next().is_none() {
        Some(score)
    } else {
        None
    }
}

/// Limits the number of options.
fn limit_options<T: Clone>(options: &[T], display_limit: Option<usize>) -> Vec<T> {
    let limit_index = if let Some(display_limit) = display_limit {
        if options.len() > display_limit {
            display_limit
        } else {
            options.len()
        }
    } else {
        options.len()
    };

    (options[..limit_index]).to_owned()
}

/// Returns a list of possible options, taking into account the complete list
/// of options, the currently selected options, and the option the user has
/// begun to type out.
fn get_possible_options(
    all_options: &[String],
    selected_options: &[String],
    next_option: &str,
    display_limit: Option<usize>,
    max_selections: Option<usize>,
) -> Vec<String> {
    if let Some(max_selections) = max_selections {
        if selected_options.len() >= max_selections {
            return Vec::new();
        }
    }

    let unselected_options = all_options
        .iter()
        .filter_map(|option| (!selected_options.contains(option)).then_some(option.to_owned()))
        .collect::<Vec<_>>();

    if next_option.is_empty() {
        return limit_options(&unselected_options, display_limit);
    }

    let mut matches = unselected_options
        .into_iter()
        .filter_map(|option| option_match(&option, next_option).map(|score| (option, score)))
        .collect::<Vec<_>>();

    matches.sort_by(|(_, score1), (_, score2)| score1.cmp(score2));

    let limited_matches = limit_options(&matches, display_limit);

    limited_matches
        .into_iter()
        .map(|(option, _)| option)
        .collect()
}

/// Chips properties.
#[derive(Properties, PartialEq, Clone)]
pub struct ChipsProps {
    /// The state of the currently selected chips.
    pub state: UseStateHandle<Vec<String>>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<Vec<String>>,
    /// The list of chip options.
    pub options: Vec<String>,
    /// The maximum number of options to display in the dropdown.
    #[prop_or_default]
    pub option_display_limit: Option<usize>,
    /// The maximum number of options that can be selected.
    #[prop_or_default]
    pub max_selections: Option<usize>,
    /// The chips input label.
    #[prop_or_default]
    pub label: AttrValue,
    /// Chips input placeholder text.
    #[prop_or_default]
    pub placeholder: AttrValue,
    /// The maximum number of characters allowed in the chip input.
    #[prop_or(524288)]
    pub max_length: usize,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the chip input is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A chip selection component.
#[function_component]
pub fn Chips(props: &ChipsProps) -> Html {
    let ChipsProps {
        state,
        on_change,
        options,
        option_display_limit,
        max_selections,
        label,
        placeholder,
        max_length,
        error,
        disabled,
    } = props.clone();

    use_effect_with_deps(
        move |new_state| on_change.emit((**new_state).clone()),
        state.clone(),
    );

    let next_chip_state = use_state(String::new);
    let next_chip = (*next_chip_state).clone();
    let id_state = use_state(new_id);
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);
    let possible_options = get_possible_options(
        &options,
        &state,
        &next_chip_state,
        option_display_limit,
        max_selections,
    );
    let oninput = {
        let next_chip_state = next_chip_state.clone();
        move |event: InputEvent| {
            let new_next_chip = input_event_value(event);
            next_chip_state.set(new_next_chip);
        }
    };
    let onfocusin = {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            dropdown_open.set(true);
        }
    };
    let onkeydown = {
        let first_option = possible_options.first().map(|option| option.to_owned());
        let state = state.clone();
        let next_chip_state = next_chip_state.clone();
        move |event: KeyboardEvent| match event.key_code() {
            13 => {
                // enter
                if let Some(ref option) = first_option {
                    let mut chips = (*state).clone();
                    chips.push(option.to_owned());
                    state.set(chips);
                    next_chip_state.set(String::new());
                }
            }
            8 => {
                // backspace
                if next_chip_state.is_empty() && !state.is_empty() {
                    let mut chips = (*state).clone();
                    chips.remove(chips.len() - 1);
                    state.set(chips);
                }
            }
            _ => {}
        }
    };

    let chip_list = (*state)
        .iter()
        .enumerate()
        .map(|(index, this_chip)| {
            let on_click = {
                let state = state.clone();
                move |_| {
                    let mut current_chips_without_this = (*state).clone();
                    current_chips_without_this.remove(index);
                    state.set(current_chips_without_this);
                }
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

    let conditional_chip_list = if (*state).is_empty() {
        html! {}
    } else {
        html! {
            <div class="base-chips-chip-list">
                {chip_list}
            </div>
        }
    };

    let chips_node = use_node_ref();
    use_click_away(chips_node.clone(), {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            dropdown_open.set(false);
        }
    });

    let chip_options = possible_options
        .iter()
        .map(|this_option| {
            let this_option = this_option.clone();
            let this_option_html = this_option.clone();
            let state = state.clone();
            let next_chip_state = next_chip_state.clone();
            let on_option_click = move |_| {
                let mut option_chips = (*state).clone();
                option_chips.push(this_option.clone());
                state.set(option_chips);
                next_chip_state.set(String::new());
            };

            html! {
                <div onclick={on_option_click} class={classes!("base-chips-option")}>
                    {this_option_html}
                </div>
            }
        })
        .collect::<Html>();

    let conditional_chip_options = if possible_options.is_empty() {
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
            <div ref={chips_node} class="base-chips">
                <div class="base-chips-inner">
                    {conditional_chip_list}
                    <input
                        type="text"
                        value={next_chip}
                        {id}
                        {oninput}
                        {onfocusin}
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
