use super::util::*;
use super::*;
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
    /// The input type.
    #[prop_or_default]
    pub input_type: InputType,
    /// The input label.
    #[prop_or_default]
    pub label: String,
    /// Input placeholder text.
    #[prop_or_default]
    pub placeholder: String,
    /// The maximum number of characters allowed.
    #[prop_or(524288)]
    pub max_length: usize,
    /// Whether the input is required to be filled out.
    #[prop_or(false)]
    pub required: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<String>,
    /// Whether the input is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// An input element.
#[function_component]
pub fn Input(props: &InputProps) -> Html {
    let InputProps {
        state,
        input_type,
        label,
        placeholder,
        max_length,
        required,
        error,
        disabled,
    } = props.clone();

    let value = (*state).clone();
    let id = new_id();
    let html_input_type = input_type.html_input_type();
    let oninput = move |event: InputEvent| {
        let new_value = input_event_value(event);
        state.set(new_value);
    };

    html! {
        <div class="base-input-container">
            <label for={id.clone()} class="base-input-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <input
                type={html_input_type}
                {value}
                {id}
                {oninput}
                {placeholder}
                {required}
                {disabled}
                maxlength={max_length.to_string()}
                class={classes!("base-input", error.clone().map(|_| "base-input-invalid"))}
            />
            <Error message={error} size={ErrorSize::Small} />
        </div>
    }
}
