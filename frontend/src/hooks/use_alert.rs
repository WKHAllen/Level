use crate::components::base::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GlobalAlertStatus {
    #[default]
    Closed,
    Opening,
    Open,
    Closing,
}

#[derive(Debug, Clone, PartialEq, Store)]
pub struct GlobalAlert {
    pub status: GlobalAlertStatus,
    pub title: String,
    pub duration: AlertDuration,
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

#[derive(Clone)]
pub struct UseAlertHandle {
    dispatch: Dispatch<GlobalAlert>,
}

impl UseAlertHandle {
    pub fn open(&self, config: UseAlert) {
        self.dispatch.reduce_mut(|alert| {
            alert.status = GlobalAlertStatus::Opening;
            alert.title = config.title;
            alert.duration = config.duration;
            alert.content = config.content;
        });
    }
}

pub struct UseAlert {
    title: String,
    duration: AlertDuration,
    content: Html,
}

#[allow(dead_code)]
impl UseAlert {
    pub fn new() -> Self {
        Self {
            title: "".to_owned(),
            duration: AlertDuration::Infinite,
            content: html! {},
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    pub fn duration(mut self, duration: AlertDuration) -> Self {
        self.duration = duration;
        self
    }

    pub fn close_after(mut self, seconds: u32) -> Self {
        self.duration = AlertDuration::Finite(seconds);
        self
    }

    pub fn html(mut self, html_content: Html) -> Self {
        self.content = html_content;
        self
    }

    pub fn text(mut self, text_content: &str) -> Self {
        self.content = html! {
            <>{text_content}</>
        };
        self
    }
}

#[hook]
pub fn use_alert() -> UseAlertHandle {
    let (_, dispatch) = use_store::<GlobalAlert>();

    UseAlertHandle { dispatch }
}
