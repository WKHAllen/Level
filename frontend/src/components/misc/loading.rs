use crate::components::base::*;
use yew::prelude::*;

/// The loading spinner size used throughout the app.
pub const APP_SPINNER_SIZE: SpinnerSize = SpinnerSize::Medium;

/// A generalized centered loading indicator.
#[function_component]
pub fn Loading() -> Html {
    html! {
        <div class="loading">
            <Spinner
                size={APP_SPINNER_SIZE}
                center={true}
            />
        </div>
    }
}
