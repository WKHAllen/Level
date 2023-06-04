use yew::prelude::*;

/// Progress bar properties.
#[derive(Properties, PartialEq, Clone)]
pub struct ProgressBarProps {
    /// The progress as a portion between 0 and 1.
    pub progress: f64,
    /// Whether the progress bar is disabled. This is only for the purpose of
    /// appearance, and has no impact on interactivity.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A progress bar component.
#[function_component]
pub fn ProgressBar(props: &ProgressBarProps) -> Html {
    let ProgressBarProps { progress, disabled } = props.clone();

    let width_percentage = progress * 100.0;
    let width_style = format!("width: {width_percentage}%;");

    html! {
        <div class={classes!("base-progress", disabled.then_some("base-progress-disabled"))}>
            <div class="base-progress-empty"></div>
            <div class="base-progress-filled" style={width_style}></div>
        </div>
    }
}
