use super::*;
use crate::hooks::use_id;
use crate::util::*;
use std::borrow::Borrow;
use std::ops::Deref;
use yew::prelude::*;

/// Shortens a number to a specified number of decimal places.
fn shorten_to(value_str: &str, decimals: u16) -> String {
    match value_str.find('.') {
        Some(index) => {
            if index + (decimals as usize) < value_str.len() {
                (value_str[..=index + (decimals as usize)]).to_owned()
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
            value_str = (value_str[1..value_str.len() - 1]).to_owned()
        } else {
            value_str = format!("-{}", &value_str[..value_str.len() - 1])
        }
    }

    if value_str.len() > 1 && value_str.starts_with('0') && !value_str.starts_with("0.") {
        value_str = (value_str[1..]).to_owned();
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
pub struct NumberState<N: Number> {
    /// The inner state string.
    state: String,
    /// The inner state value.
    value: N,
    /// The minimum value.
    min: N,
    /// The maximum value.
    max: N,
    /// The maximum number of digits after the decimal.
    decimals: u16,
}

impl<N: Number> NumberState<N> {
    /// Creates a new number state.
    pub fn new(value: N) -> Self {
        Self::default().value(value)
    }

    /// Sets the state value.
    pub fn value(mut self, value: N) -> Self {
        self.state = value.to_string();
        self.value = value;
        self
    }

    /// Sets the minimum value.
    pub fn min(mut self, min: N) -> Self {
        self.min = min;
        self
    }

    /// Sets the maximum value.
    pub fn max(mut self, max: N) -> Self {
        self.max = max;
        self
    }

    /// Sets the maximum number of digits after the decimal.
    pub fn decimals(mut self, decimals: u16) -> Self {
        self.decimals = decimals;
        self
    }

    /// Sets the inner state.
    fn set(&mut self, new_value_str: &str) {
        let new_value_transformed = transform_number(new_value_str, self.decimals);
        let maybe_new_value = parse_number(&new_value_transformed, self.min, self.max);

        if let Some((new_value, update_repr)) = maybe_new_value {
            if !update_repr {
                self.state = new_value_transformed;
            } else {
                self.state = new_value.to_string();
            }

            self.value = parse_number(&self.state, self.min, self.max).unwrap().0;
        }
    }
}

impl<N: Number> Deref for NumberState<N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<N: Number> Borrow<N> for NumberState<N> {
    fn borrow(&self) -> &N {
        &self.value
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
            value: N::default(),
            min: N::NUMBER_MIN,
            max: N::NUMBER_MAX,
            decimals: u16::MAX,
        }
    }
}

impl<N: Number> From<N> for NumberState<N> {
    fn from(value: N) -> Self {
        Self::default().value(value)
    }
}

/// Input properties.
#[derive(Properties, PartialEq, Clone)]
pub struct NumberInputProps<N: Number> {
    /// The number input state.
    pub state: UseStateHandle<NumberState<N>>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<N>,
    /// The number input label.
    #[prop_or_default]
    pub label: AttrValue,
    /// Number input placeholder text.
    #[prop_or_default]
    pub placeholder: AttrValue,
    /// Whether the input is required to be filled out.
    #[prop_or(false)]
    pub required: bool,
    /// Whether to compact the element into a smaller space.
    #[prop_or(false)]
    pub compact: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the input is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The number input node ref.
    #[prop_or_default]
    pub node: NodeRef,
}

/// An input element.
#[function_component]
pub fn NumberInput<N: Number + 'static>(props: &NumberInputProps<N>) -> Html {
    let NumberInputProps {
        state,
        on_change,
        label,
        placeholder,
        required,
        compact,
        error,
        disabled,
        node,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(***new_state));

    let value_str = (*state).to_string();
    let id_state = use_id();
    let id = (*id_state).clone();
    let trigger = use_force_update();

    let oninput = move |event: InputEvent| {
        let new_value_str = input_event_value(event);
        let mut new_state = (*state).clone();
        new_state.set(&new_value_str);
        state.set(new_state);
        trigger.force_update(); // necessary in case the state has not changed
    };

    html! {
        <div class={classes!("base-input-container", compact.then_some("base-input-container-compact"), disabled.then_some("base-input-container-disabled"))}>
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
                class={classes!("base-input", error.as_ref().map(|_| "base-input-invalid"))}
                ref={node}
            />
            <Error message={error} size={ErrorSize::Small} class="base-input-error" />
        </div>
    }
}
