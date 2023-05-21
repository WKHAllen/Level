use super::util::*;
use super::*;
use yew::prelude::*;

/// Shortens a number to a specified number of decimal places.
fn shorten_to(value_str: &str, decimals: u16) -> String {
    match value_str.find('.') {
        Some(index) => {
            if index + (decimals as usize) < value_str.len() {
                (&value_str[..=index + (decimals as usize)]).to_owned()
            } else {
                value_str.to_owned()
            }
        }
        None => value_str.to_owned(),
    }
}

/// Transforms a string representation of a number as needed.
fn transform_number(value_str: &str, decimals: u16) -> String {
    let mut value_str = shorten_to(value_str, decimals);

    if value_str == "-" {
        value_str = "".to_owned();
    }

    if value_str.ends_with('-') {
        if value_str.starts_with('-') {
            value_str = (&value_str[1..value_str.len() - 1]).to_owned()
        } else {
            value_str = format!("-{}", &value_str[..value_str.len() - 1])
        }
    }

    if value_str.len() > 1 && value_str.starts_with("0") && !value_str.starts_with("0.") {
        value_str = (&value_str[1..]).to_owned();
    }

    if value_str.len() > 2 && value_str.starts_with("-0") && !value_str.starts_with("-0.") {
        value_str = format!("-{}", (&value_str[2..]));
    }

    value_str
}

/// Parses the value of a string representation of a number in a text input box.
fn parse_number_value<N: Number>(value_str: &str, min: N, max: N) -> Option<(N, bool)> {
    match value_str.parse::<N>() {
        Ok(value) => {
            if value < min {
                Some((min, true))
            } else if value > max {
                Some((max, true))
            } else {
                Some((value, false))
            }
        }
        Err(_) => None,
    }
}

/// Parses a string representation of a number in a text input box.
fn parse_number<N: Number>(value_str: &str, min: N, max: N) -> Option<(N, bool)> {
    if value_str.is_empty() {
        Some((N::default(), true))
    } else if N::DECIMAL && value_str.ends_with('.') && value_str.matches('.').count() == 1 {
        parse_number_value(&value_str[..value_str.len() - 1], min, max)
    } else {
        parse_number_value(value_str, min, max)
    }
}

/// A wrapper around a number state.
#[derive(Debug, Clone, PartialEq)]
struct NumberState<N: Number> {
    /// The inner state string.
    state: String,
    /// The minimum value.
    min: N,
    /// The maximum value.
    max: N,
    /// The maximum number of digits after the decimal.
    decimals: u16,
}

impl<N: Number> NumberState<N> {
    /// Creates a new number state.
    pub fn new(value: N, min: N, max: N, decimals: u16) -> Self {
        Self {
            state: value.to_string(),
            min,
            max,
            decimals,
        }
    }

    /// Gets the inner value.
    pub fn get(&self) -> N {
        parse_number(&self.state, self.min, self.max).unwrap().0
    }

    /// Sets the inner state.
    pub fn set(&mut self, new_value_str: &str) {
        let new_value_transformed = transform_number(new_value_str, self.decimals);
        let maybe_new_value = parse_number(&new_value_transformed, self.min, self.max);

        if let Some((new_value, update_repr)) = maybe_new_value {
            if !update_repr {
                self.state = new_value_transformed;
            } else {
                self.state = new_value.to_string();
            }
        }
    }
}

impl<N: Number> ToString for NumberState<N> {
    fn to_string(&self) -> String {
        if self.state.is_empty() {
            N::default().to_string()
        } else {
            self.state.clone()
        }
    }
}

impl<N: Number> Default for NumberState<N> {
    fn default() -> Self {
        Self {
            state: String::new(),
            min: N::NUMBER_MIN,
            max: N::NUMBER_MAX,
            decimals: u16::MAX,
        }
    }
}

/// Input properties.
#[derive(Properties, PartialEq, Clone)]
pub struct NumberInputProps<N: Number> {
    /// The number input state.
    pub state: UseStateHandle<N>,
    /// The number input label.
    #[prop_or_default]
    pub label: String,
    /// Number input placeholder text.
    #[prop_or_default]
    pub placeholder: String,
    /// The minimum value.
    #[prop_or(N::NUMBER_MIN)]
    pub min: N,
    /// The maximum value.
    #[prop_or(N::NUMBER_MAX)]
    pub max: N,
    /// The maximum number of decimal places.
    #[prop_or(u16::MAX)]
    pub decimals: u16,
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
pub fn NumberInput<N: Number + 'static>(props: &NumberInputProps<N>) -> Html {
    let NumberInputProps {
        state,
        label,
        placeholder,
        min,
        max,
        decimals,
        required,
        error,
        disabled,
    } = props.clone();

    let state_update = use_state(|| Option::<(N, NumberState<N>)>::None);
    let number_state = use_state(|| NumberState::new(*state, min, max, decimals));

    if let Some((new_state, new_state_str)) = &*state_update {
        state.set(new_state.to_owned());
        number_state.set(new_state_str.to_owned());
        state_update.set(None);
    }

    let value_str = (*number_state).to_string();
    let id_state = use_state(|| new_id());
    let id = (*id_state).clone();

    let oninput = move |event: InputEvent| {
        let new_value_str = input_event_value(event);
        let mut new_number_state = (*number_state).clone();
        new_number_state.set(&new_value_str);
        let new_state = new_number_state.get();
        let new_state_str = NumberState::<N>::default();

        number_state.set(new_state_str);
        state_update.set(Some((new_state, new_number_state)));
    };

    html! {
        <div class={classes!("base-input-container", disabled.then_some("base-input-container-disabled"))}>
            <label for={id.clone()} class="base-input-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <input
                type="text"
                value={value_str}
                {id}
                {oninput}
                {placeholder}
                {required}
                {disabled}
                class={classes!("base-input", error.clone().map(|_| "base-input-invalid"))}
            />
            <Error message={error} size={ErrorSize::Small} />
        </div>
    }
}
