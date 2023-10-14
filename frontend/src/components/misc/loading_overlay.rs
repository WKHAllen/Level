use crate::components::base::*;
use crate::components::misc::APP_SPINNER_SIZE;
use yew::prelude::*;

/// Loading indicator properties.
#[derive(Properties, PartialEq, Clone)]
pub struct LoadingOverlayProps {
    /// The overlay state.
    pub state: UseStateHandle<bool>,
}

/// A generalized centered loading indicator.
#[function_component]
pub fn LoadingOverlay(props: &LoadingOverlayProps) -> Html {
    let LoadingOverlayProps { state } = props.clone();

    html! {
        <div class={classes!("loading-overlay", state.then_some("loading-overlay-open"))}>
            <Spinner
                size={APP_SPINNER_SIZE}
                center={true}
            />
        </div>
    }
}
