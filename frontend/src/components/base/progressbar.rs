use yew::prelude::*;

/// Progress bar properties.
#[derive(Properties, PartialEq, Clone)]
pub struct ProgressBarProps {
    /// The progress as a portion between 0 and 1.
    pub progress: f64,
}

/// A progress bar component.
#[function_component]
pub fn ProgressBar(props: &ProgressBarProps) -> Html {
    let ProgressBarProps { progress } = props.clone();

    let width_percentage = progress * 100.0;
    let width_style = format!("width: {}%;", width_percentage);

    html! {
        <div class="base-progress">
            <div class="base-progress-empty"></div>
            <div class="base-progress-filled" style={width_style}></div>
        </div>
    }
}
