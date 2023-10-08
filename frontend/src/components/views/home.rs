use crate::components::base::*;
use crate::hooks::*;
use crate::view::View;
use yew::prelude::*;

/// The home page view.
#[function_component]
pub fn Home() -> Html {
    let view = use_view();
    let on_click = move |_| view.set(View::Open);

    html! {
        <div class="home">
            <div class="home-title">
                <h1>{"level"}</h1>
                <span>{"A secure app for tracking personal finances."}</span>
                <Button text="Begin" {on_click} />
            </div>
        </div>
    }
}
