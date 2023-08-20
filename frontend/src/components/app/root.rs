use crate::components::base::Demo;
use crate::hooks::use_theme;
use yew::prelude::*;

/// The root element of the application.
#[function_component]
pub fn Root() -> Html {
    // initialize the theme
    _ = use_theme();

    html! {
        <>
            <Demo />
        </>
    }
}
