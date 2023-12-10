use super::*;
use crate::hooks::*;
use yew::prelude::*;
use yew_hooks::use_click_away;

pub use common::SelectOptions;

/// Position of a select popup.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SelectPopupPosition {
    /// Position the popup above.
    Above,
    /// Position the popup below.
    #[default]
    Below,
}

impl SelectPopupPosition {
    /// Gets the name of the position.
    pub fn position_name(&self) -> &'static str {
        match *self {
            Self::Above => "above",
            Self::Below => "below",
        }
    }
}

/// Select properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    /// The selection state.
    pub state: UseStateHandle<usize>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<usize>,
    /// The list of select options.
    pub options: Vec<String>,
    /// The selection label.
    #[prop_or_default]
    pub label: AttrValue,
    /// The positioning of the popup.
    #[prop_or_default]
    pub position: SelectPopupPosition,
    /// Whether a selection is required.
    #[prop_or(false)]
    pub required: bool,
    /// Whether to compact the element into a smaller space.
    #[prop_or(false)]
    pub compact: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the selection is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The select button node ref.
    #[prop_or_default]
    pub node: NodeRef,
}

/// A select component.
#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let SelectProps {
        state,
        on_change,
        options,
        label,
        position,
        required,
        compact,
        error,
        disabled,
        node,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(**new_state));

    let id_state = use_id();
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);

    let on_button_click = {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            if !disabled {
                dropdown_open.set(!*dropdown_open);
            }
        }
    };

    let select_node = use_node_ref();
    use_click_away(select_node.clone(), {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            dropdown_open.set(false);
        }
    });

    let selected_option = match options.get(*state) {
        Some(selected) => html! { <>{selected}</> },
        None => html! { <>{"Select..."}</> },
    };

    let option_selections = options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let on_option_click = {
                let state = state.clone();
                let dropdown_open = dropdown_open.clone();
                move |_| {
                    state.set(index);
                    dropdown_open.set(false);
                }
            };

            html! {
                <div onclick={on_option_click} class="base-select-option">
                    {option}
                </div>
            }
        })
        .collect::<Html>();

    let position_class = format!("base-select-{}", position.position_name());

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-select-container", compact.then_some("base-select-container-compact"), disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div ref={select_node} class={classes!("base-select", position_class)}>
                <button
                    {id}
                    onclick={on_button_click}
                    {disabled}
                    class={classes!("base-select-button", error.as_ref().map(|_| "base-select-button-invalid"))}
                    ref={node}
                >
                    <div class="base-select-button-selection">
                        {selected_option}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div ref={popup_node} class="base-select-popup">
                        {option_selections}
                    </div>
                </div>
            </div>
            <Error message={error} size={ErrorSize::Small} class="base-select-error" />
        </div>
    }
}

/// Select with null option properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SelectNullableProps {
    /// The selection state.
    pub state: UseStateHandle<Option<usize>>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<Option<usize>>,
    /// The list of select options.
    pub options: Vec<String>,
    /// The selection label.
    #[prop_or_default]
    pub label: AttrValue,
    /// The null option label.
    #[prop_or("Select...".into())]
    pub null_label: AttrValue,
    /// The positioning of the popup.
    #[prop_or_default]
    pub position: SelectPopupPosition,
    /// Whether the selection is required to be in a non-null state.
    #[prop_or(false)]
    pub required: bool,
    /// Whether to compact the element into a smaller space.
    #[prop_or(false)]
    pub compact: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the selection is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The select button node ref.
    #[prop_or_default]
    pub node: NodeRef,
}

/// A select component with a null option.
#[function_component]
pub fn SelectNullable(props: &SelectNullableProps) -> Html {
    let SelectNullableProps {
        state,
        on_change,
        options,
        label,
        null_label,
        position,
        required,
        compact,
        error,
        disabled,
        node,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(**new_state));

    let id_state = use_id();
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);

    let on_button_click = {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            if !disabled {
                dropdown_open.set(!*dropdown_open);
            }
        }
    };

    let select_node = use_node_ref();
    use_click_away(select_node.clone(), {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            dropdown_open.set(false);
        }
    });

    let selected_option = if let Some(state_value) = *state {
        match options.get(state_value) {
            Some(selected) => html! { <>{selected}</> },
            None => html! { <>{null_label.clone()}</> },
        }
    } else {
        html! {
            <>{null_label.clone()}</>
        }
    };

    let option_selections = options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let on_option_click = {
                let state = state.clone();
                let dropdown_open = dropdown_open.clone();
                move |_| {
                    state.set(Some(index));
                    dropdown_open.set(false);
                }
            };

            html! {
                <div onclick={on_option_click} class="base-select-option">
                    {option}
                </div>
            }
        })
        .collect::<Html>();

    let on_null_click = {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            state.set(None);
            dropdown_open.set(false);
        }
    };

    let position_class = format!("base-select-{}", position.position_name());

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-select-container", "base-select-nullable", compact.then_some("base-select-container-compact"), disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div ref={select_node} class={classes!("base-select", position_class)}>
                <button
                    {id}
                    onclick={on_button_click}
                    {disabled}
                    class={classes!("base-select-button", error.as_ref().map(|_| "base-select-button-invalid"))}
                    ref={node}
                >
                    <div class="base-select-button-selection">
                        {selected_option}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div ref={popup_node} class="base-select-popup">
                        <div onclick={on_null_click} class="base-select-option">
                            <>{null_label}</>
                        </div>
                        {option_selections}
                    </div>
                </div>
            </div>
            <Error message={error} size={ErrorSize::Small} class="base-select-error" />
        </div>
    }
}

/// Select with enum state properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SelectEnumProps<T: SelectOptions> {
    /// The selection state.
    pub state: UseStateHandle<T>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<T>,
    /// The selection label.
    #[prop_or_default]
    pub label: AttrValue,
    /// The positioning of the popup.
    #[prop_or_default]
    pub position: SelectPopupPosition,
    /// Whether a selection is required.
    #[prop_or(false)]
    pub required: bool,
    /// Whether to compact the element into a smaller space.
    #[prop_or(false)]
    pub compact: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the selection is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The select button node ref.
    #[prop_or_default]
    pub node: NodeRef,
}

/// A select component using enum variants as options.
#[function_component]
pub fn SelectEnum<T: SelectOptions + 'static>(props: &SelectEnumProps<T>) -> Html {
    let SelectEnumProps {
        state,
        on_change,
        label,
        position,
        required,
        compact,
        error,
        disabled,
        node,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(**new_state));

    let id_state = use_id();
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);

    let on_button_click = {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            if !disabled {
                dropdown_open.set(!*dropdown_open);
            }
        }
    };

    let select_node = use_node_ref();
    use_click_away(select_node.clone(), {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            dropdown_open.set(false);
        }
    });

    let selected_option = html! {
        <>{(*state).to_string()}</>
    };

    let option_selections = T::options()
        .into_iter()
        .enumerate()
        .map(|(index, option)| {
            let on_option_click = {
                let state = state.clone();
                let dropdown_open = dropdown_open.clone();
                move |_| {
                    state.set(T::from_index(index));
                    dropdown_open.set(false);
                }
            };

            html! {
                <div onclick={on_option_click} class="base-select-option">
                    {option}
                </div>
            }
        })
        .collect::<Html>();

    let position_class = format!("base-select-{}", position.position_name());

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-select-container", compact.then_some("base-select-container-compact"), disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div ref={select_node} class={classes!("base-select", position_class)}>
                <button
                    {id}
                    onclick={on_button_click}
                    {disabled}
                    class={classes!("base-select-button", error.as_ref().map(|_| "base-select-button-invalid"))}
                    ref={node}
                >
                    <div class="base-select-button-selection">
                        {selected_option}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div ref={popup_node} class="base-select-popup">
                        {option_selections}
                    </div>
                </div>
            </div>
            <Error message={error} size={ErrorSize::Small} class="base-select-error" />
        </div>
    }
}

/// Select with nullable enum state properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SelectNullableEnumProps<T: SelectOptions> {
    /// The selection state.
    pub state: UseStateHandle<Option<T>>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<Option<T>>,
    /// The selection label.
    #[prop_or_default]
    pub label: AttrValue,
    /// The null option label.
    #[prop_or("Select...".into())]
    pub null_label: AttrValue,
    /// The positioning of the popup.
    #[prop_or_default]
    pub position: SelectPopupPosition,
    /// Whether the selection is required to be in a non-null state.
    #[prop_or(false)]
    pub required: bool,
    /// Whether to compact the element into a smaller space.
    #[prop_or(false)]
    pub compact: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the selection is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The select button node ref.
    #[prop_or_default]
    pub node: NodeRef,
}

/// A select component using nullable enum variants as options.
#[function_component]
pub fn SelectNullableEnum<T: SelectOptions + 'static>(props: &SelectNullableEnumProps<T>) -> Html {
    let SelectNullableEnumProps {
        state,
        on_change,
        label,
        null_label,
        position,
        required,
        compact,
        error,
        disabled,
        node,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(**new_state));

    let id_state = use_id();
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);

    let on_button_click = {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            if !disabled {
                dropdown_open.set(!*dropdown_open);
            }
        }
    };

    let select_node = use_node_ref();
    use_click_away(select_node.clone(), {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            dropdown_open.set(false);
        }
    });

    let selected_option = if let Some(state_value) = *state {
        html! {
            <>{state_value.to_string()}</>
        }
    } else {
        html! {
            <>{null_label.clone()}</>
        }
    };

    let option_selections = T::options()
        .into_iter()
        .enumerate()
        .map(|(index, option)| {
            let on_option_click = {
                let state = state.clone();
                let dropdown_open = dropdown_open.clone();
                move |_| {
                    state.set(Some(T::from_index(index)));
                    dropdown_open.set(false);
                }
            };

            html! {
                <div onclick={on_option_click} class="base-select-option">
                    {option}
                </div>
            }
        })
        .collect::<Html>();

    let on_null_click = {
        let dropdown_open = dropdown_open.clone();
        move |_| {
            state.set(None);
            dropdown_open.set(false);
        }
    };

    let position_class = format!("base-select-{}", position.position_name());

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-select-container", "base-select-nullable", compact.then_some("base-select-container-compact"), disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div ref={select_node} class={classes!("base-select", position_class)}>
                <button
                    {id}
                    onclick={on_button_click}
                    {disabled}
                    class={classes!("base-select-button", error.as_ref().map(|_| "base-select-button-invalid"))}
                    ref={node}
                >
                    <div class="base-select-button-selection">
                        {selected_option}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div ref={popup_node} class="base-select-popup">
                        <div onclick={on_null_click} class="base-select-option">
                            <>{null_label}</>
                        </div>
                        {option_selections}
                    </div>
                </div>
            </div>
            <Error message={error} size={ErrorSize::Small} class="base-select-error" />
        </div>
    }
}
