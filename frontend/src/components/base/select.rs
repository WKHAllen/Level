use super::*;
use crate::hooks::*;
use yew::prelude::*;
use yew_hooks::use_click_away;

pub use common::SelectOptions;

/// Select option properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SelectOptionProps {
    /// Whether the select option is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    #[prop_or_default]
    pub children: Children,
}

/// A select option component.
#[function_component]
pub fn SelectOption(props: &SelectOptionProps) -> Html {
    let SelectOptionProps { children, .. } = props.clone();

    html! {
        <>
            {children}
        </>
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
    /// The selection label.
    #[prop_or_default]
    pub label: AttrValue,
    /// Whether a selection is required.
    #[prop_or(false)]
    pub required: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the selection is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The select button node ref.
    #[prop_or_default]
    pub node: NodeRef,
    /// Child elements.
    #[prop_or_default]
    pub children: ChildrenWithProps<SelectOption>,
}

/// A select component.
#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let SelectProps {
        state,
        on_change,
        label,
        required,
        error,
        disabled,
        node,
        children,
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

    let selected_child = if *state < children.len() {
        children.iter().nth(*state).unwrap().into()
    } else {
        html! {
            <SelectOption>{"Select..."}</SelectOption>
        }
    };

    let new_children = children
        .iter()
        .enumerate()
        .map(|(index, child)| {
            let SelectOptionProps {
                disabled: child_disabled,
                children: child_children,
            } = (*child.props).clone();

            let on_child_click = {
                let state = state.clone();
                let dropdown_open = dropdown_open.clone();
                move |_| {
                    if !child_disabled {
                        state.set(index);
                        dropdown_open.set(false);
                    }
                }
            };

            html! {
                <div onclick={on_child_click} class={classes!("base-select-option", child_disabled.then_some("base-select-option-disabled"))}>
                    {child_children}
                </div>
            }
        })
        .collect::<Html>();

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-select-container", disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div ref={select_node} class="base-select">
                <button
                    {id}
                    onclick={on_button_click}
                    {disabled}
                    class={classes!("base-select-button", error.as_ref().map(|_| "base-select-button-invalid"))}
                    ref={node}
                >
                    <div class="base-select-button-selection">
                        {selected_child}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div ref={popup_node} class="base-select-popup">
                        {new_children}
                    </div>
                </div>
            </div>
            <Error message={error} size={ErrorSize::Small} />
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
    /// The selection label.
    #[prop_or_default]
    pub label: AttrValue,
    /// The null option label.
    #[prop_or("Select...".into())]
    pub null_label: AttrValue,
    /// Whether the selection is required to be in a non-null state.
    #[prop_or(false)]
    pub required: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the selection is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The select button node ref.
    #[prop_or_default]
    pub node: NodeRef,
    /// Child elements.
    #[prop_or_default]
    pub children: ChildrenWithProps<SelectOption>,
}

/// A select component with a null option.
#[function_component]
pub fn SelectNullable(props: &SelectNullableProps) -> Html {
    let SelectNullableProps {
        state,
        on_change,
        label,
        null_label,
        required,
        error,
        disabled,
        node,
        children,
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

    let selected_child = if let Some(state_value) = *state {
        if state_value < children.len() {
            children.iter().nth(state_value).unwrap().into()
        } else {
            html! {
                <SelectOption>{null_label.clone()}</SelectOption>
            }
        }
    } else {
        html! {
            <SelectOption>{null_label.clone()}</SelectOption>
        }
    };

    let new_children = children
        .iter()
        .enumerate()
        .map(|(index, child)| {
            let SelectOptionProps {
                disabled: child_disabled,
                children: child_children,
            } = (*child.props).clone();

            let on_child_click = {
                let state = state.clone();
                let dropdown_open = dropdown_open.clone();
                move |_| {
                    if !child_disabled {
                        state.set(Some(index));
                        dropdown_open.set(false);
                    }
                }
            };

            html! {
                <div onclick={on_child_click} class={classes!("base-select-option", child_disabled.then_some("base-select-option-disabled"))}>
                    {child_children}
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

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-select-container", disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div ref={select_node} class="base-select">
                <button
                    {id}
                    onclick={on_button_click}
                    {disabled}
                    class={classes!("base-select-button", error.as_ref().map(|_| "base-select-button-invalid"))}
                    ref={node}
                >
                    <div class="base-select-button-selection">
                        {selected_child}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div ref={popup_node} class="base-select-popup">
                        <div onclick={on_null_click} class="base-select-option">
                            <SelectOption>{null_label}</SelectOption>
                        </div>
                        {new_children}
                    </div>
                </div>
            </div>
            <Error message={error} size={ErrorSize::Small} />
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
    /// Whether a selection is required.
    #[prop_or(false)]
    pub required: bool,
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
        required,
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

    let selected_child = html! {
        <SelectOption>{(*state).to_string()}</SelectOption>
    };

    let new_children = T::options()
        .into_iter()
        .enumerate()
        .map(|(index, option)| {
            let on_child_click = {
                let state = state.clone();
                let dropdown_open = dropdown_open.clone();
                move |_| {
                    state.set(T::from_index(index));
                    dropdown_open.set(false);
                }
            };

            html! {
                <div onclick={on_child_click} class="base-select-option">
                    {option}
                </div>
            }
        })
        .collect::<Html>();

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-select-container", disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div ref={select_node} class="base-select">
                <button
                    {id}
                    onclick={on_button_click}
                    {disabled}
                    class={classes!("base-select-button", error.as_ref().map(|_| "base-select-button-invalid"))}
                    ref={node}
                >
                    <div class="base-select-button-selection">
                        {selected_child}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div ref={popup_node} class="base-select-popup">
                        {new_children}
                    </div>
                </div>
            </div>
            <Error message={error} size={ErrorSize::Small} />
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
    /// Whether the selection is required to be in a non-null state.
    #[prop_or(false)]
    pub required: bool,
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
        required,
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

    let selected_child = if let Some(state_value) = *state {
        html! {
            <SelectOption>{state_value.to_string()}</SelectOption>
        }
    } else {
        html! {
            <SelectOption>{null_label.clone()}</SelectOption>
        }
    };

    let new_children = T::options()
        .into_iter()
        .enumerate()
        .map(|(index, option)| {
            let on_child_click = {
                let state = state.clone();
                let dropdown_open = dropdown_open.clone();
                move |_| {
                    state.set(Some(T::from_index(index)));
                    dropdown_open.set(false);
                }
            };

            html! {
                <div onclick={on_child_click} class="base-select-option">
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

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-select-container", disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div ref={select_node} class="base-select">
                <button
                    {id}
                    onclick={on_button_click}
                    {disabled}
                    class={classes!("base-select-button", error.as_ref().map(|_| "base-select-button-invalid"))}
                    ref={node}
                >
                    <div class="base-select-button-selection">
                        {selected_child}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div ref={popup_node} class="base-select-popup">
                        <div onclick={on_null_click} class="base-select-option">
                            <SelectOption>{null_label}</SelectOption>
                        </div>
                        {new_children}
                    </div>
                </div>
            </div>
            <Error message={error} size={ErrorSize::Small} />
        </div>
    }
}
