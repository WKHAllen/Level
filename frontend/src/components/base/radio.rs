use crate::hooks::use_id;
use crate::util::*;
use yew::prelude::*;

/// The orientation of a radio group.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RadioGroupOrientation {
    /// Horizontally oriented.
    Horizontal,
    /// Vertically oriented.
    #[default]
    Vertical,
}

impl RadioGroupOrientation {
    /// Gets the name of the orientation.
    pub fn orientation_name(&self) -> &'static str {
        match *self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Radio group properties.
#[derive(Properties, PartialEq, Clone)]
pub struct RadioGroupProps {
    /// The radio group state.
    pub state: UseStateHandle<Option<usize>>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<Option<usize>>,
    /// The list of radio options.
    pub options: Vec<String>,
    /// The orientation of the radio group.
    #[prop_or_default]
    pub orientation: RadioGroupOrientation,
    /// Whether a selection is required.
    #[prop_or(false)]
    pub required: bool,
    /// Whether the radio group is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A radio group component.
#[function_component]
pub fn RadioGroup(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        state,
        on_change,
        options,
        orientation,
        required,
        disabled,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(**new_state));

    let name_state = use_id();
    let name = (*name_state).clone();
    let id_states = use_state(|| {
        vec![false; options.len()]
            .into_iter()
            .map(|_| new_id())
            .collect::<Vec<_>>()
    });
    let ids = (*id_states).clone();
    let orientation_class = format!("base-radio-group-{}", orientation.orientation_name());

    let radio_options = options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let id = ids[index].clone();
            let checked = state.filter(|value| *value == index).is_some();
            let oninput = {
                let state = state.clone();
                move |_| {
                    state.set(Some(index));
                }
            };

            html! {
                <div class={classes!("base-radio-option", disabled.then_some("base-radio-option-disabled"))}>
                    <input
                        type="radio"
                        id={id.clone()}
                        name={name.clone()}
                        value={index.to_string()}
                        {oninput}
                        {checked}
                        {required}
                        {disabled}
                        class="base-radio-input"
                    />
                    <label for={id} class="base-radio-label">{option}</label>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class={classes!("base-radio-group", orientation_class)}>
            {radio_options}
        </div>
    }
}
