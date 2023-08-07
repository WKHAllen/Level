use super::*;
use gloo_timers::callback::Timeout;
use yew::prelude::*;

/// Alert duration.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AlertDuration {
    /// A finite duration in seconds.
    Finite(u32),
    /// An infinite duration.
    #[default]
    Infinite,
}

impl AlertDuration {
    /// Is the duration infinite?
    pub fn is_infinite(&self) -> bool {
        match *self {
            Self::Finite(_) => false,
            Self::Infinite => true,
        }
    }
}

/// Alert properties.
#[derive(Properties, PartialEq, Clone)]
pub struct AlertProps {
    /// The alert open state.
    pub state: UseStateHandle<bool>,
    /// The alert title.
    #[prop_or_default]
    pub title: AttrValue,
    /// The duration of time for which the alert should exist.
    #[prop_or_default]
    pub duration: AlertDuration,
    /// The callback called with the alert closing state. Receives `true` if
    /// the alert was closed manually and `false` otherwise.
    #[prop_or_default]
    pub on_close: Callback<bool>,
    /// Elements within the alert.
    pub children: Children,
}

/// An alert component.
#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    let AlertProps {
        state,
        title,
        duration,
        on_close,
        children,
    } = props.clone();

    let timeout_close_callback = on_close.clone();
    let timeout_close_state = state.clone();
    let timeout_state = use_state(|| None);
    let timeout_state_close = timeout_state.clone();

    if *state && !duration.is_infinite() && timeout_state.is_none() {
        timeout_state.set(match duration {
            AlertDuration::Finite(seconds) => {
                if *state {
                    Some(Timeout::new(seconds * 1000, move || {
                        timeout_close_callback.emit(false);
                        timeout_close_state.set(false);
                        timeout_state_close.set(None);
                    }))
                } else {
                    None
                }
            }
            AlertDuration::Infinite => None,
        });
    }

    let x_close = {
        let x_close_callback = on_close; // .clone();
        let x_close_state = state.clone();
        let x_close_timeout_state = timeout_state; // .clone();
        move |_| {
            x_close_callback.emit(true);
            x_close_state.set(false);
            x_close_timeout_state.set(None); // timeout is cancelled when dropped
        }
    };

    html! {
        <div class={classes!("base-alert", (*state).then_some("base-alert-open"))}>
            <div class="base-alert-inner">
                <div class="base-alert-header">
                    <div class="base-alert-header-space"></div>
                    <h4 class="base-alert-title">{title}</h4>
                    <IconButton
                        name="xmark-solid"
                        size={IconButtonSize::Medium}
                        on_click={x_close}
                    />
                </div>
                <div class="base-alert-body">
                    {children}
                </div>
            </div>
        </div>
    }
}
