use crate::components::{Greeting, Quote};
use yew::prelude::*;

/// The root element of the application.
#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <Greeting name="Will" />
            <Quote />
        </div>
    }
}
