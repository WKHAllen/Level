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
    /// The slider state.
    pub state: UseStateHandle<usize>,
    /// The slider label.
    #[prop_or_default]
    pub label: String,
    /// An optional error message.
    #[prop_or_default]
    pub error: Option<String>,
    /// Whether the slider is disabled.
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
        error,
        disabled,
        children,
    } = props.clone();

    let id_state = use_state(|| new_id());
    let id = (*id_state).clone();
    let dropdown_open = use_state(|| false);
    let dropdown_open_focus_in = dropdown_open.clone();
    let dropdown_open_focus_out = dropdown_open.clone();

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
            let onmousedown = move |_| {
                if !child_disabled {
                    child_state.set(index);
                }
            };

            html! {
                <div class={classes!("base-select-option", child_disabled.then_some("base-select-option-disabled"))} {onmousedown}>
                    {child_children}
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class={classes!("base-select-container", disabled.then_some("base-select-container-disabled"), (*dropdown_open).then_some("base-select-container-open"))}>
            <label for={id.clone()} class="base-select-label">{label}</label>
            <div class="base-select">
                <button
                    {id}
                    {onfocusin}
                    {onfocusout}
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
