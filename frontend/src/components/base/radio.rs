use super::util::*;
use yew::prelude::*;

/// Radio button properties.
#[derive(Properties, PartialEq, Clone)]
pub struct RadioButtonProps {
    /// Whether the radio button is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    pub children: Children,
}

/// A radio button component.
#[function_component]
pub fn RadioButton(props: &RadioButtonProps) -> Html {
    let RadioButtonProps { children, .. } = props.clone();

    html! {
        <>
            {children}
        </>
    }
}

/// The orientation of a radio group.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RadioGroupOrientation {
    /// Horizontallly oriented.
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
    /// The orientation of the radio group.
    #[prop_or_default]
    pub orientation: RadioGroupOrientation,
    /// Whether a selection is required.
    #[prop_or(false)]
    pub required: bool,
    /// Whether the radio group is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    pub children: ChildrenWithProps<RadioButton>,
}

/// A radio group component.
#[function_component]
pub fn RadioGroup(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        state,
        orientation,
        required,
        disabled,
        children,
    } = props.clone();

    let name_state = use_state(new_id);
    let name = (*name_state).clone();
    let id_states = use_state(|| {
        vec![false; children.len()]
            .into_iter()
            .map(|_| new_id())
            .collect::<Vec<_>>()
    });
    let ids = (*id_states).clone();
    let orientation_class = format!("base-radio-group-{}", orientation.orientation_name());

    let new_children = children
        .iter()
        .enumerate()
        .map(|(index, child)| {
            let RadioButtonProps {
                disabled: child_disabled,
                children: child_children,
            } = (*child.props).clone();

            let id = ids[index].clone();
            let checked = state.filter(|value| *value == index).is_some();
            let this_disabled = disabled || child_disabled;
            let oninput = {
                let state = state.clone();
                move |_| {
                    state.set(Some(index));
                }
            };

            html! {
                <div class={classes!("base-radio-option", this_disabled.then_some("base-radio-option-disabled"))}>
                    <input
                        type="radio"
                        id={id.clone()}
                        name={name.clone()}
                        value={index.to_string()}
                        {oninput}
                        {checked}
                        {required}
                        disabled={this_disabled}
                        class="base-radio-input"
                    />
                    <label for={id} class="base-radio-label">{child_children}</label>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class={classes!("base-radio-group", orientation_class)}>
            {new_children}
        </div>
    }
}
