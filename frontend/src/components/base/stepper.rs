use super::*;
use yew::prelude::*;

/// Step properties.
#[derive(Properties, PartialEq, Clone)]
pub struct StepProps {
    /// Whether the step is valid, enabling progression to the next step.
    #[prop_or(true)]
    pub valid: bool,
    /// Child elements.
    pub children: Children,
}

/// A step component.
#[function_component]
pub fn Step(props: &StepProps) -> Html {
    let StepProps { children, .. } = props.clone();

    html! {
        <>
            {children}
        </>
    }
}

/// The state of a stepper.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepperState {
    /// The stepper is in progress.
    InProgress(usize),
    /// The stepper is complete.
    Complete,
}

#[allow(dead_code)]
impl StepperState {
    /// Gets the index of the currently active step.
    pub fn step(&self) -> Option<usize> {
        match *self {
            Self::InProgress(step) => Some(step),
            Self::Complete => None,
        }
    }

    /// Is the stepper currently in progress?
    pub fn in_progress(&self) -> bool {
        matches!(self, Self::InProgress(_))
    }

    /// Is the stepper currently complete?
    pub fn complete(&self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Constructs a new state with the previous step.
    pub fn prev(&self) -> Self {
        match *self {
            Self::InProgress(step) => Self::InProgress(if step > 0 { step - 1 } else { 0 }),
            Self::Complete => Self::Complete,
        }
    }

    /// Constructs a new state with next step.
    pub fn next(&self) -> Self {
        match *self {
            Self::InProgress(step) => Self::InProgress(step + 1),
            Self::Complete => Self::Complete,
        }
    }

    /// Constructs a new completed state.
    pub fn finish(&self) -> Self {
        Self::Complete
    }
}

impl Default for StepperState {
    fn default() -> Self {
        Self::InProgress(0)
    }
}

/// Stepper properties.
#[derive(Properties, PartialEq, Clone)]
pub struct StepperProps {
    /// The stepper state.
    pub state: UseStateHandle<StepperState>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<StepperState>,
    /// The callback called when the stepper has been completed.
    #[prop_or_default]
    pub on_complete: Callback<()>,
    /// A title to display above the stepper.
    #[prop_or_default]
    pub title: String,
    /// Child elements.
    pub children: ChildrenWithProps<Step>,
}

/// A stepper component.
#[function_component]
pub fn Stepper(props: &StepperProps) -> Html {
    let StepperProps {
        state,
        on_change,
        on_complete,
        title,
        children,
    } = props.clone();

    use_effect_with_deps(
        move |new_state| {
            on_change.emit(**new_state);

            if new_state.complete() {
                on_complete.emit(());
            }
        },
        state.clone(),
    );

    let (current_step, prev_button, next_button) = match *state {
        StepperState::InProgress(step) => {
            if let Some(current) = children.iter().nth(step) {
                let valid = current.props.valid;

                let current_step = html! {
                    <>
                        {current}
                    </>
                };

                let prev_button = if step > 0 {
                    let on_click = {
                        let state = state.clone();
                        move |_| {
                            state.set(state.prev());
                        }
                    };

                    html! {
                        <Button
                            text="Back"
                            {on_click}
                        />
                    }
                } else {
                    html! {
                        <div></div>
                    }
                };

                let next_button = if step < children.len() - 1 {
                    let on_click = {
                        let state = state.clone();
                        move |_| {
                            state.set(state.next());
                        }
                    };

                    html! {
                        <Button
                            text="Next"
                            disabled={!valid}
                            {on_click}
                        />
                    }
                } else {
                    let on_click = {
                        let state = state.clone();
                        move |_| {
                            state.set(state.finish());
                        }
                    };

                    html! {
                        <Button
                            text="Finish"
                            disabled={!valid}
                            {on_click}
                        />
                    }
                };

                (current_step, prev_button, next_button)
            } else {
                state.set(state.finish());

                (html! {}, html! {}, html! {})
            }
        }
        StepperState::Complete => (html! {}, html! {}, html! {}),
    };

    if state.in_progress() {
        html! {
            <div class="base-stepper">
                <div class="base-stepper-title">
                    <h3>{title}</h3>
                </div>
                <div class="base-stepper-step">
                    {current_step}
                </div>
                <div class="base-stepper-actions">
                    {prev_button}
                    {next_button}
                </div>
            </div>
        }
    } else {
        html! {
            <div class="base-stepper"></div>
        }
    }
}
