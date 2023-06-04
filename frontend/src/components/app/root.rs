use crate::components::base::Demo;
use yew::prelude::*;

/// The root element of the application.
#[function_component]
pub fn Root() -> Html {
    html! {
        <>
            <Demo />
        </>
    }
}
