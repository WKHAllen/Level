use crate::components::base::*;
use yew::prelude::*;
use yewdux::prelude::*;

/// The current alert status.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GlobalAlertStatus {
    /// The alert is closed.
    #[default]
    Closed,
    /// The alert is opening.
    Opening,
    /// The alert is open.
    Open,
    /// The alert is closing.
    Closing,
}

/// A global application alert.
#[derive(Debug, Clone, PartialEq, Store)]
pub struct GlobalAlert {
    /// The current alert status.
    pub status: GlobalAlertStatus,
    /// The alert title.
    pub title: String,
    /// The alert duration.
    pub duration: AlertDuration,
    /// The HTML alert body.
    pub content: Html,
}

impl Default for GlobalAlert {
    fn default() -> Self {
        Self {
            status: GlobalAlertStatus::Closed,
            title: "".to_owned(),
            duration: AlertDuration::Infinite,
            content: html! {},
        }
    }
}

/// A handle to the global application alert.
#[derive(Clone)]
pub struct UseAlertHandle {
    /// The dispatcher for the global alert.
    dispatch: Dispatch<GlobalAlert>,
}

impl UseAlertHandle {
    /// Open the global alert with the given config.
    pub fn open(&self, config: UseAlert) {
        self.dispatch.reduce_mut(|alert| {
            alert.status = GlobalAlertStatus::Opening;
            alert.title = config.title;
            alert.duration = config.duration;
            alert.content = config.content;
        });
    }
}

/// Configuration for the [`use_alert`] hook.
pub struct UseAlert {
    /// The alert title.
    title: String,
    /// The alert duration.
    duration: AlertDuration,
    /// The HTML alert body.
    content: Html,
}

#[allow(dead_code)]
impl UseAlert {
    /// Creates a new `use_alert` configuration.
    pub fn new() -> Self {
        Self {
            title: "".to_owned(),
            duration: AlertDuration::Infinite,
            content: html! {},
        }
    }

    /// Sets the alert title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    /// Sets the alert duration.
    pub fn duration(mut self, duration: AlertDuration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the number of seconds after which the alert will automatically
    /// close.
    pub fn close_after(mut self, seconds: u32) -> Self {
        self.duration = AlertDuration::Finite(seconds);
        self
    }

    /// Sets the alert body HTML.
    pub fn html(mut self, html_content: Html) -> Self {
        self.content = html_content;
        self
    }

    /// Sets the alert body text.
    pub fn text(mut self, text_content: &str) -> Self {
        self.content = html! {
            <>{text_content}</>
        };
        self
    }
}

/// Gets a handle to the global application alert popup.
#[hook]
pub fn use_alert() -> UseAlertHandle {
    let (_, dispatch) = use_store::<GlobalAlert>();

    UseAlertHandle { dispatch }
}
