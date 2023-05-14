use super::*;
use yew::prelude::*;

/// A demo of the base components.
#[function_component]
pub fn Demo() -> Html {
    let input_state = use_state(|| String::new());
    let input_value = (*input_state).clone();
    let textarea_state = use_state(|| String::new());
    let textarea_value = (*textarea_state).clone();
    let textarea_state1 = use_state(|| String::new());
    let textarea_state2 = use_state(|| String::new());
    let button_state = use_state(|| ButtonStyle::Primary);
    let button_state_primary = button_state.clone();
    let button_state_secondary = button_state.clone();
    let button_state_transparent = button_state.clone();
    let button_state_danger = button_state.clone();
    let button_value = *button_state;

    html! {
        <div class="base-demo">
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Error"}</span>
                <Error message="A large error message" size={ErrorSize::Larger} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Input"}</span>
                <Input state={input_state.clone()} label="Input label" placeholder="Placeholder!" required={true} error={input_value.is_empty().then_some("Please enter a value")} />
                <span>{"Value: "}{input_value}</span>
                <Input state={input_state} label="Disabled input" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Textarea"}</span>
                <TextArea state={textarea_state.clone()} label="Textarea label" placeholder="Placeholder!" required={true} error={textarea_value.is_empty().then_some("Please enter a value")} />
                <span>{"Value: "}{textarea_value}</span>
                <TextArea state={textarea_state} label="Disabled textarea" disabled={true} resize={TextAreaResize::Horizontal} />
                <TextArea state={textarea_state1} label="Vertical resize" resize={TextAreaResize::Vertical} />
                <TextArea state={textarea_state2} label="Full resize" resize={TextAreaResize::Both} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Button"}</span>
                <Button text="Primary" on_click={move |_| button_state_primary.set(ButtonStyle::Primary)} />
                <Button text="Secondary" style={ButtonStyle::Secondary} on_click={move |_| button_state_secondary.set(ButtonStyle::Secondary)} />
                <Button text="Transparent" style={ButtonStyle::Transparent} on_click={move |_| button_state_transparent.set(ButtonStyle::Transparent)} />
                <Button text="Danger" style={ButtonStyle::Danger} on_click={move |_| button_state_danger.set(ButtonStyle::Danger)} />
                <Button text="Disabled" style={*button_state} disabled={true} />
                <span>{"Last clicked: "}{button_value.style_name()}</span>
            </div>
        </div>
    }
}
