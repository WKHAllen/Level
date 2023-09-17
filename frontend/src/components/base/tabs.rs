use yew::prelude::*;

/// Tab properties.
#[derive(Properties, PartialEq, Clone)]
pub struct TabProps {
    /// The tab label.
    pub label: AttrValue,
    /// Whether the tab is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Child elements.
    #[prop_or_default]
    pub children: Children,
}

/// A tab component.
#[function_component]
pub fn Tab(props: &TabProps) -> Html {
    let TabProps { children, .. } = props.clone();

    html! {
        <>
            {children}
        </>
    }
}

/// The tab alignment style.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TabAlign {
    /// Left aligned tabs.
    Left,
    /// Right aligned tabs.
    Right,
    /// Centered tabs.
    Center,
    /// All tabs equally stretched to the width of the container.
    #[default]
    Stretch,
}

impl TabAlign {
    /// Gets the name of the alignment style.
    pub fn alignment_name(&self) -> &'static str {
        match *self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Center => "center",
            Self::Stretch => "stretch",
        }
    }
}

/// Tab group properties.
#[derive(Properties, PartialEq, Clone)]
pub struct TabGroupProps {
    /// The tab state.
    pub state: UseStateHandle<usize>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<usize>,
    /// The tab alignment style.
    #[prop_or_default]
    pub align: TabAlign,
    /// Child elements.
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab>,
}

/// A tab group component.
#[function_component]
pub fn TabGroup(props: &TabGroupProps) -> Html {
    let TabGroupProps {
        state,
        on_change,
        align,
        children,
    } = props.clone();

    use_effect_with_deps(move |new_state| on_change.emit(**new_state), state.clone());

    let num_children = children.len();
    let alignment_class = format!("base-tab-group-tabs-{}", align.alignment_name());

    let labels = children
        .iter()
        .enumerate()
        .map(|(index, child)| {
            let state = state.clone();
            let label = &child.props.label;
            let disabled = child.props.disabled;
            let selected = index == (*state);
            let selection_class = if selected { "base-tab-group-tab-indicator-selected" } else { "base-tab-group-tab-indicator-unselected" };
            let onclick = move |_| {
                if !disabled {
                    state.set(index);
                }
            };

            html! {
                <div
                    class={classes!("base-tab-group-tab", selected.then_some("base-tab-group-tab-selected"), disabled.then_some("base-tab-group-tab-disabled"))}
                    {onclick}
                >
                    <div class="base-tab-group-tab-label">{label}</div>
                    <div class={classes!("base-tab-group-tab-indicator", selection_class)}></div>
                </div>
            }
        })
        .collect::<Html>();

    if let Some(current) = children.into_iter().nth(*state) {
        html! {
            <div class="base-tab-group">
                <div class={classes!("base-tab-group-tabs", alignment_class)}>
                    {labels}
                </div>
                <div class="base-tab-group-body">
                    {current}
                </div>
            </div>
        }
    } else {
        state.set(*state % num_children);

        html! {}
    }
}
