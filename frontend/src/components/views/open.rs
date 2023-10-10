use crate::components::base::*;
use yew::prelude::*;

/// The page view to open a save.
#[function_component]
pub fn Open() -> Html {
    html! {
        <div class="view open">
            <Frame>
                <span>{"Open placeholder"}</span>
            </Frame>
        </div>
    }
}
