use super::util::*;
use super::*;
use yew::prelude::*;

/// Select option properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SelectOptionProps {
    /// Whether the select option is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
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
    /// The selection label.
    #[prop_or_default]
    pub label: String,
    /// Whether a selection is required.
    #[prop_or(false)]
    pub required: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<String>,
    /// Whether the selection is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    pub children: ChildrenWithProps<SelectOption>,
}

/// A select component.
#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let SelectProps {
        state,
        label,
        required,
        error,
        disabled,
        children,
    } = props.clone();

    let id_state = use_state(|| new_id());
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);
    let dropdown_open_focus_in = dropdown_open.clone();
    let dropdown_open_focus_out = dropdown_open.clone();
    let dropdown_open_mouse_down = dropdown_open.clone();

    let onfocusin = move |_| {
        if !*dropdown_open_focus_in {
            dropdown_open_focus_in.set(true);
        }
    };
    let onfocusout = move |_| {
        if *dropdown_open_focus_out {
            dropdown_open_focus_out.set(false);
        }
    };
    let onmousedown = move |_| {
        if !disabled {
            dropdown_open_mouse_down.set(!*dropdown_open_mouse_down);
        }
    };

    let selected_child = if *state < children.len() {
        children.iter().skip(*state).next().clone().unwrap().into()
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

            let child_state = state.clone();
            let child_onmousedown = move |_| {
                if !child_disabled {
                    child_state.set(index);
                }
            };

            html! {
                <div class={classes!("base-select-option", child_disabled.then_some("base-select-option-disabled"))} onmousedown={child_onmousedown}>
                    {child_children}
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class={classes!("base-select-container", disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div class="base-select">
                <button
                    {id}
                    {onfocusin}
                    {onfocusout}
                    {onmousedown}
                    {disabled}
                    class={classes!("base-select-button", error.clone().map(|_| "base-select-button-invalid"))}
                >
                    <div class="base-select-button-selection">
                        {selected_child}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div class="base-select-popup">
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
pub struct SelectWithNullProps {
    /// The selection state.
    pub state: UseStateHandle<Option<usize>>,
    /// The selection label.
    #[prop_or_default]
    pub label: String,
    /// The null option label.
    #[prop_or("Select...".to_owned())]
    pub null_label: String,
    /// Whether the selection is required to be in a non-null state.
    #[prop_or(false)]
    pub required: bool,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<String>,
    /// Whether the selection is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    pub children: ChildrenWithProps<SelectOption>,
}

/// A select component with a null option.
#[function_component]
pub fn SelectWithNull(props: &SelectWithNullProps) -> Html {
    let SelectWithNullProps {
        state,
        label,
        null_label,
        required,
        error,
        disabled,
        children,
    } = props.clone();

    let id_state = use_state(|| new_id());
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);
    let dropdown_open_focus_in = dropdown_open.clone();
    let dropdown_open_focus_out = dropdown_open.clone();
    let dropdown_open_mouse_down = dropdown_open.clone();

    let onfocusin = move |_| {
        if !*dropdown_open_focus_in {
            dropdown_open_focus_in.set(true);
        }
    };
    let onfocusout = move |_| {
        if *dropdown_open_focus_out {
            dropdown_open_focus_out.set(false);
        }
    };
    let onmousedown = move |_| {
        if !disabled {
            dropdown_open_mouse_down.set(!*dropdown_open_mouse_down);
        }
    };

    let selected_child = if let Some(state_value) = *state {
        if state_value < children.len() {
            children
                .iter()
                .skip(state_value)
                .next()
                .clone()
                .unwrap()
                .into()
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

            let child_state = state.clone();
            let child_onmousedown = move |_| {
                if !child_disabled {
                    child_state.set(Some(index));
                }
            };

            html! {
                <div class={classes!("base-select-option", child_disabled.then_some("base-select-option-disabled"))} onmousedown={child_onmousedown}>
                    {child_children}
                </div>
            }
        })
        .collect::<Html>();

    let null_onmousedown = {
        let null_state = state.clone();
        move |_| {
            null_state.set(None);
        }
    };

    html! {
        <div class={classes!("base-select-container", disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div class="base-select">
                <button
                    {id}
                    {onfocusin}
                    {onfocusout}
                    {onmousedown}
                    {disabled}
                    class={classes!("base-select-button", error.clone().map(|_| "base-select-button-invalid"))}
                >
                    <div class="base-select-button-selection">
                        {selected_child}
                    </div>
                    <Icon name="angle-down-solid" {disabled} class="base-select-button-icon" />
                </button>
                <div class="base-select-dropdown">
                    <div class="base-select-popup">
                        <div class="base-select-option" onmousedown={null_onmousedown}>
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
