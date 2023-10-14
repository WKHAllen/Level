use super::*;
use yew::prelude::*;

/// Dialog size.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DialogSize {
    Small,
    #[default]
    Medium,
    Large,
    Max,
    Auto,
}

impl DialogSize {
    /// Gets the name of the dialog size.
    pub fn size_name(&self) -> &'static str {
        match *self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Max => "max",
            Self::Auto => "auto",
        }
    }
}

/// Dialog action buttons layout.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DialogActionsLayout {
    /// Left-aligned actions.
    Left,
    /// Right-aligned actions.
    #[default]
    Right,
    /// Actions spaced across the line.
    Spaced,
}

impl DialogActionsLayout {
    /// Gets the name of the actions layout.
    pub fn layout_name(&self) -> &'static str {
        match *self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Spaced => "spaced",
        }
    }
}

/// Dialog properties.
#[derive(Properties, PartialEq, Clone)]
pub struct DialogProps {
    /// The dialog open state.
    pub state: UseStateHandle<bool>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// The dialog size.
    #[prop_or_default]
    pub size: DialogSize,
    /// The dialog title.
    #[prop_or_default]
    pub title: AttrValue,
    /// The ok button label. Will not be created if empty.
    #[prop_or_default]
    pub ok_label: AttrValue,
    /// The cancel button label. Will not be created if empty.
    #[prop_or_default]
    pub cancel_label: AttrValue,
    /// The callback called with the dialog closing state. Receives `true` if
    /// the ok button was clicked and `false` otherwise.
    #[prop_or_default]
    pub on_close: Callback<bool>,
    /// The layout of action buttons.
    #[prop_or_default]
    pub actions_layout: DialogActionsLayout,
    /// Elements within the dialog.
    #[prop_or_default]
    pub children: Children,
}

/// A dialog component.
#[function_component]
pub fn Dialog(props: &DialogProps) -> Html {
    let DialogProps {
        state,
        on_change,
        size,
        title,
        ok_label,
        cancel_label,
        on_close,
        actions_layout,
        children,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(**new_state));

    let size_class = format!("base-dialog-{}", size.size_name());
    let actions_layout_class = format!("base-dialog-actions-{}", actions_layout.layout_name());

    let x_close = {
        let on_close = on_close.clone();
        let state = state.clone();
        move |_| {
            on_close.emit(false);
            state.set(false);
        }
    };
    let ok_close = {
        let on_close = on_close.clone();
        let state = state.clone();
        move |_| {
            on_close.emit(true);
            state.set(false);
        }
    };
    let cancel_close = {
        let on_close = on_close.clone();
        let state = state.clone();
        move |_| {
            on_close.emit(false);
            state.set(false);
        }
    };

    let mouse_in_state = use_state(|| false);
    let dialog_mousein = {
        let mouse_in_state = mouse_in_state.clone();
        move |_| {
            mouse_in_state.set(true);
        }
    };
    let dialog_mouseout = {
        let mouse_in_state = mouse_in_state.clone();
        move |_| {
            mouse_in_state.set(false);
        }
    };
    let container_click = {
        let state = state.clone();
        move |_| {
            if !*mouse_in_state {
                on_close.emit(false);
                state.set(false);
            }
        }
    };

    html! {
        <div
            class={classes!("base-dialog-container", (*state).then_some("base-dialog-container-open"))}
            onclick={container_click}
        >
            <div
                class={classes!("base-dialog", size_class)}
                onmouseenter={dialog_mousein}
                onmouseleave={dialog_mouseout}
            >
                <div class="base-dialog-inner">
                    <div class="base-dialog-header">
                        <div class="base-dialog-header-space"></div>
                        <h3 class="base-dialog-title">{title}</h3>
                        <IconButton
                            name="xmark-solid"
                            size={IconButtonSize::Medium}
                            on_click={x_close}
                        />
                    </div>
                    <div class="base-dialog-body">
                        {children}
                    </div>
                    <div class={classes!("base-dialog-actions", actions_layout_class)}>
                        {(!cancel_label.is_empty()).then_some(html! {
                            <Button
                                text={cancel_label}
                                style={ButtonStyle::Transparent}
                                on_click={cancel_close}
                            />
                        })}
                        {(!ok_label.is_empty()).then_some(html! {
                            <Button
                                text={ok_label}
                                style={ButtonStyle::Primary}
                                on_click={ok_close}
                            />
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}
