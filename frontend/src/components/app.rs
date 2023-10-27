use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::prelude::*;

/// The primary color of all UI elements within the app.
const APP_PRIMARY_COLOR: (u8, u8, u8) = (0, 111, 0);

/// The root element of the application.
#[function_component]
pub fn App() -> Html {
    let demo = use_demo();
    let (_, dispatch_theme) = use_theme();
    let view = use_view();
    let (alert, dispatch_alert) = use_store::<GlobalAlert>();
    let alert_state = use_state(|| false);

    use_effect_once(move || {
        dispatch_theme.reduce_mut(|theme| theme.set_primary_color(APP_PRIMARY_COLOR));
        || ()
    });

    match alert.status {
        GlobalAlertStatus::Opening => {
            dispatch_alert.reduce_mut(|alert| alert.status = GlobalAlertStatus::Open);
            alert_state.set(true);
        }
        GlobalAlertStatus::Closing => {
            dispatch_alert.reduce_mut(|alert| alert.status = GlobalAlertStatus::Closed);
            alert_state.set(false);
        }
        _ => {}
    }

    let on_alert_close = {
        let dispatch_alert = dispatch_alert.clone();
        move |_| dispatch_alert.reduce_mut(|alert| alert.status = GlobalAlertStatus::Closing)
    };

    match demo {
        UseDemoHandle::Unresolved => html! {
            <div class="app">
                <Loading />
            </div>
        },
        UseDemoHandle::Resolved(true) => html! { <Demo /> },
        UseDemoHandle::Resolved(false) => html! {
            <div class="app">
                <div class="main">
                    {view.html_view()}
                </div>
                <Alert
                    state={alert_state}
                    title={alert.title.clone()}
                    duration={alert.duration}
                    on_close={on_alert_close}
                >
                    {alert.content.clone()}
                </Alert>
            </div>
        },
    }
}
