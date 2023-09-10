use super::*;
use crate::hooks::use_id;
use crate::util::*;
use yew::prelude::*;

/// The type of input element.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum InputType {
    /// Standard text input.
    #[default]
    Text,
    /// Email address input.
    Email,
    /// Telephone number input.
    Tel,
    /// URL input.
    Url,
    /// Password input.
    Password,
}

impl InputType {
    /// Gets the HTML input element type corresponding to the current input type.
    pub fn html_input_type(&self) -> &'static str {
        match *self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Password => "password",
        }
    }
}

/// Input properties.
#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    /// The input state.
    pub state: UseStateHandle<String>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<String>,
    /// The input type.
    #[prop_or_default]
    pub input_type: InputType,
    /// The input label.
    #[prop_or_default]
    pub label: AttrValue,
    /// Input placeholder text.
    #[prop_or_default]
    pub placeholder: AttrValue,
    /// The maximum number of characters allowed.
    #[prop_or(524288)]
    pub max_length: usize,
    /// The callback called when the enter key is pressed.
    #[prop_or_default]
    pub on_submit: Callback<()>,
    /// Whether the input is required to be filled out.
    #[prop_or(false)]
    pub required: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the input is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// An input element.
#[function_component]
pub fn Input(props: &InputProps) -> Html {
    let InputProps {
        state,
        on_change,
        input_type,
        label,
        placeholder,
        max_length,
        on_submit,
        required,
        error,
        disabled,
    } = props.clone();

    use_effect_with_deps(
        move |new_state| on_change.emit((**new_state).clone()),
        state.clone(),
    );

    let value = (*state).clone();
    let id_state = use_id();
    let id = (*id_state).clone();
    let html_input_type = input_type.html_input_type();
    let oninput = move |event: InputEvent| {
        let new_value = input_event_value(event);
        state.set(new_value);
    };
    let onkeydown = move |event: KeyboardEvent| {
        if event.key_code() == 13 {
            // enter
            on_submit.emit(());
        }
    };

    html! {
        <div class={classes!("base-input-container", disabled.then_some("base-input-container-disabled"))}>
            <label for={id.clone()} class="base-input-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <input
                type={html_input_type}
                {value}
                {id}
                {oninput}
                {onkeydown}
                {placeholder}
                {required}
                {disabled}
                maxlength={max_length.to_string()}
                class={classes!("base-input", error.as_ref().map(|_| "base-input-invalid"))}
            />
            <Error message={error} size={ErrorSize::Small} />
        </div>
    }
}
