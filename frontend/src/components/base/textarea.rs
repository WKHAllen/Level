use super::*;
use crate::hooks::use_id;
use crate::util::*;
use yew::prelude::*;

/// Textarea resize options.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TextAreaResize {
    /// No resize.
    #[default]
    None,
    /// Horizontal resize only.
    Horizontal,
    /// Vertical resize only.
    Vertical,
    /// Both horizontal and vertical resize.
    Both,
}

impl TextAreaResize {
    /// Gets the name of the resize option.
    pub fn resize_option_name(&self) -> &'static str {
        match *self {
            Self::None => "none",
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
            Self::Both => "both",
        }
    }
}

/// Textarea properties.
#[derive(Properties, PartialEq, Clone)]
pub struct TextAreaProps {
    /// The textarea state.
    pub state: UseStateHandle<String>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<String>,
    /// The textarea label.
    #[prop_or_default]
    pub label: AttrValue,
    /// Textarea placeholder text.
    #[prop_or_default]
    pub placeholder: AttrValue,
    /// The maximum number of characters allowed.
    #[prop_or(524288)]
    pub max_length: usize,
    /// Whether the textarea is required to be filled out.
    #[prop_or(false)]
    pub required: bool,
    /// The number of rows displayed by default.
    #[prop_or(3)]
    pub rows: usize,
    /// In what way the textarea can be resized.
    #[prop_or_default]
    pub resize: TextAreaResize,
    /// Whether to compact the element into a smaller space.
    #[prop_or(false)]
    pub compact: bool,
    /// The icon to use for the optional action button.
    #[prop_or_default]
    pub action_icon: Option<AttrValue>,
    /// The action button callback.
    #[prop_or_default]
    pub on_action: Callback<()>,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the textarea is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The textarea node ref.
    #[prop_or_default]
    pub node: NodeRef,
}

/// A textarea element.
#[function_component]
pub fn TextArea(props: &TextAreaProps) -> Html {
    let TextAreaProps {
        state,
        on_change,
        label,
        placeholder,
        max_length,
        required,
        rows,
        resize,
        compact,
        action_icon,
        on_action,
        error,
        disabled,
        node,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| {
        on_change.emit((**new_state).clone())
    });

    let value = (*state).clone();
    let id_state = use_id();
    let id = (*id_state).clone();
    let resize_class = format!("base-textarea-resize-{}", resize.resize_option_name());
    let oninput = move |event: InputEvent| {
        let new_value = textarea_event_value(event);
        state.set(new_value);
    };

    let optional_action = match action_icon {
        Some(action_icon) => html! {
            <IconButton
                name={action_icon}
                size={IconButtonSize::Small}
                on_click={on_action}
            />
        },
        None => html! {},
    };

    html! {
        <div class={classes!("base-textarea-container", compact.then_some("base-textarea-container-compact"), disabled.then_some("base-textarea-container-disabled"))}>
            <div class="base-textarea-label-container">
                <label for={id.clone()} class="base-textarea-label">
                    {label}
                    <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
                </label>
                {optional_action}
            </div>
            <div class="base-textarea-box-container">
                <textarea
                    rows={rows.to_string()}
                    {value}
                    {id}
                    {oninput}
                    {placeholder}
                    {required}
                    {disabled}
                    maxlength={max_length.to_string()}
                    class={classes!("base-textarea", resize_class, error.as_ref().map(|_| "base-textarea-invalid"))}
                    ref={node}
                />
            </div>
            <Error message={error} size={ErrorSize::Small} class="base-textarea-error" />
        </div>
    }
}
