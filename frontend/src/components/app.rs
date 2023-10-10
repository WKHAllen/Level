use crate::components::base::*;
use crate::hooks::*;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// The primary color of all UI elements within the app.
const APP_PRIMARY_COLOR: (u8, u8, u8) = (0, 111, 0);

/// The root element of the application.
#[function_component]
pub fn App() -> Html {
    let demo = use_demo();

    let (_, dispatch_theme) = use_theme();

    let view = use_view();

    use_effect_once(move || {
        dispatch_theme.reduce_mut(|theme| theme.set_primary_color(APP_PRIMARY_COLOR));
        || ()
    });

    match demo {
        UseDemoHandle::Unresolved => html! { <Spinner /> },
        UseDemoHandle::Resolved(true) => html! { <Demo /> },
        UseDemoHandle::Resolved(false) => html! {
            <div class="app">
                <div class="main">
                    {view.html_view()}
                </div>
            </div>
        },
    }
}
