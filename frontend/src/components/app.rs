use crate::hooks::*;
use yew::prelude::*;
use yew_hooks::prelude::*;

const APP_PRIMARY_COLOR: (u8, u8, u8) = (0, 111, 0);

/// The root element of the application.
#[function_component]
pub fn App() -> Html {
    let (_, dispatch_theme) = use_theme();

    let view = use_view();

    use_effect_once(move || {
        dispatch_theme.reduce_mut(|theme| theme.set_primary_color(APP_PRIMARY_COLOR));
        || ()
    });

    html! {
        <div class="app">
            <div class="main">
                {view.html_view()}
            </div>
        </div>
    }
}
