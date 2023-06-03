use super::*;
use chrono::{Datelike, NaiveDate};
use yew::prelude::*;

/// A demo of the base components.
#[function_component]
pub fn Demo() -> Html {
    let input_state = use_state(|| "Input value".to_owned());
    let input_value = (*input_state).clone();
    let textarea_state = use_state(|| "Textarea value".to_owned());
    let textarea_value = (*textarea_state).clone();
    let textarea_state1 = use_state(|| String::new());
    let textarea_state2 = use_state(|| String::new());
    let numberinput_int_state = use_state(|| 3u16);
    let numberinput_int_value = *numberinput_int_state;
    let numberinput_float_state = use_state(|| 1.618f64);
    let numberinput_float_value = *numberinput_float_state;
    let button_state = use_state(|| ButtonStyle::Primary);
    let button_state_primary = button_state.clone();
    let button_state_secondary = button_state.clone();
    let button_state_transparent = button_state.clone();
    let button_state_danger = button_state.clone();
    let button_value = *button_state;
    let checkbox_state = use_state(|| true);
    let checkbox_value = *checkbox_state;
    let switch_state = use_state(|| true);
    let switch_value = *switch_state;
    let radio_state = use_state(|| None);
    let radio_value = *radio_state;
    let slider_int_state = use_state(|| 3u8);
    let slider_int_value = *slider_int_state;
    let slider_float_state = use_state(|| 1.6f32);
    let slider_float_value = *slider_float_state;
    let icon_button_state = use_state(|| 0usize);
    let icon_button_small_state = icon_button_state.clone();
    let icon_button_medium_state = icon_button_state.clone();
    let icon_button_large_state = icon_button_state.clone();
    let icon_button_value = *icon_button_state;
    let select_state = use_state(|| 0);
    let select_value = *select_state;
    let select_with_null_state = use_state(|| None);
    let select_with_null_value = *select_with_null_state;
    let dialog_close_state = use_state(|| None);
    let dialog_small_state = use_state(|| false);
    let dialog_small_button_state = dialog_small_state.clone();
    let dialog_small_close_state = dialog_close_state.clone();
    let dialog_medium_state = use_state(|| false);
    let dialog_medium_button_state = dialog_medium_state.clone();
    let dialog_medium_close_state = dialog_close_state.clone();
    let dialog_large_state = use_state(|| false);
    let dialog_large_button_state = dialog_large_state.clone();
    let dialog_large_close_state = dialog_close_state.clone();
    let dialog_max_state = use_state(|| false);
    let dialog_max_button_state = dialog_max_state.clone();
    let dialog_max_close_state = dialog_close_state.clone();
    let dialog_auto_state = use_state(|| false);
    let dialog_auto_button_state = dialog_auto_state.clone();
    let dialog_auto_close_state = dialog_close_state.clone();
    let dialog_select_state = use_state(|| 0);
    let alert_close_state = use_state(|| None);
    let alert_finite_state = use_state(|| false);
    let alert_finite_button_state = alert_finite_state.clone();
    let alert_finite_close_state = alert_close_state.clone();
    let alert_infinite_state = use_state(|| false);
    let alert_infinite_button_state = alert_infinite_state.clone();
    let alert_infinite_close_state = alert_close_state.clone();
    let card_state = use_state(|| None);
    let card_interactive_state = card_state.clone();
    let card_not_interactive_state = card_state.clone();
    let chips_state = use_state(|| {
        vec!["Java", "Go", "Rust"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>()
    });
    let chips_value = (*chips_state).clone();
    let chip_options = vec![
        "C",
        "C++",
        "C#",
        "Java",
        "JavaScript/TypeScript",
        "Python",
        "Go",
        "Rust",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect::<Vec<_>>();
    let datepicker_state = use_state(|| None);
    let datepicker_value = (*datepicker_state).clone();
    let date_min = NaiveDate::from_ymd_opt(2023, 03, 21).unwrap();
    let date_max = NaiveDate::from_ymd_opt(2026, 03, 21).unwrap();
    let date_error = (*datepicker_state)
        .as_ref()
        .map(|date: &NaiveDate| {
            (date.month() == 2).then_some("Please pick a month other than February".to_owned())
        })
        .flatten();

    html! {
        <div class="base-demo">
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Error"}</span>
                <Error message="The smallest error message" size={ErrorSize::Smaller} />
                <Error message="The small error message" size={ErrorSize::Small} />
                <Error message="The medium size error message" size={ErrorSize::Medium} />
                <Error message="The large error message" size={ErrorSize::Large} />
                <Error message="The largest error message" size={ErrorSize::Larger} />
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
                <span class="base-demo-item-label">{"Number input"}</span>
                <NumberInput<u16> state={numberinput_int_state.clone()} label="Int number input label" placeholder="Placeholder!" min={0} max={100} required={true} error={(numberinput_int_value == 3).then_some("How about something other than 3")} />
                <span>{"Value: "}{numberinput_int_value}</span>
                <NumberInput<f64> state={numberinput_float_state} label="Float number input label" placeholder="Placeholder!" min={-5.0} max={5.0} decimals={5} required={true} error={(numberinput_float_value == 3.14).then_some("No pi, please")} />
                <span>{"Value: "}{numberinput_float_value}</span>
                <NumberInput<u16> state={numberinput_int_state} label="Disabled number input" disabled={true} />
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
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Checkbox"}</span>
                <Checkbox state={checkbox_state.clone()} label="Checkbox label" />
                <span>{"Value: "}{checkbox_value.to_string()}</span>
                <Checkbox state={checkbox_state} label="Disabled checkbox" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Switch"}</span>
                <Switch state={switch_state.clone()} label="Switch label" />
                <span>{"Value: "}{switch_value.to_string()}</span>
                <Switch state={switch_state} label="Disabled switch" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Radio group"}</span>
                <RadioGroup state={radio_state.clone()}>
                    <RadioButton>{"Option 1"}</RadioButton>
                    <RadioButton>{"Option 2"}</RadioButton>
                    <RadioButton>{"Option 3"}</RadioButton>
                    <RadioButton disabled={true}>{"Option 4"}</RadioButton>
                </RadioGroup>
                <span>{"Value: "}{radio_value.map(|x| x.to_string()).unwrap_or("None".to_owned())}</span>
                <RadioGroup state={radio_state} orientation={RadioGroupOrientation::Horizontal} disabled={true}>
                    <RadioButton>{"Option 1"}</RadioButton>
                    <RadioButton>{"Option 2"}</RadioButton>
                    <RadioButton>{"Option 3"}</RadioButton>
                </RadioGroup>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Slider"}</span>
                <Slider<u8> state={slider_int_state.clone()} min={1} max={9} step={2} label="Int slider label" />
                <span>{"Value: "}{slider_int_value.to_string()}</span>
                <Slider<f32> state={slider_float_state.clone()} min={-10.0} max={10.0} step={0.1} label="Float slider label" />
                <span>{"Value: "}{slider_float_value.to_string()}</span>
                <Slider<u8> state={slider_int_state} min={1} max={17} label="Disabled slider" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Icon"}</span>
                <Icon name="xmark-solid" size={IconSize::Small} />
                <Icon name="xmark-solid" size={IconSize::Medium} />
                <Icon name="xmark-solid" size={IconSize::Large} />
                <Icon name="xmark-solid" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Icon button"}</span>
                <IconButton name="xmark-solid" size={IconButtonSize::Small} on_click={move |_| icon_button_small_state.set(icon_button_value + 1)} />
                <IconButton name="xmark-solid" size={IconButtonSize::Medium} on_click={move |_| icon_button_medium_state.set(icon_button_value + 1)} />
                <IconButton name="xmark-solid" size={IconButtonSize::Large} on_click={move |_| icon_button_large_state.set(icon_button_value + 1)} />
                <span>{"Icon button has been clicked "}{icon_button_value}{" times"}</span>
                <IconButton name="xmark-solid" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Select"}</span>
                <Select state={select_state.clone()} label="Select label" required={true} error={(select_value == 3).then_some("This option isn't available for the disabled select box below")}>
                    <SelectOption>{"Option 1"}</SelectOption>
                    <SelectOption>{"Option 2"}</SelectOption>
                    <SelectOption>{"Option 3"}</SelectOption>
                    <SelectOption>{"Option 4"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 5 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 6 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 7 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 8 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 9 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 10 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 11 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 12 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 13 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 14 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 15 (disabled)"}</SelectOption>
                </Select>
                <span>{"Value: "}{select_value.to_string()}</span>
                <Select state={select_state} label="Disabled select label" disabled={true}>
                    <SelectOption>{"Option 1"}</SelectOption>
                    <SelectOption>{"Option 2"}</SelectOption>
                    <SelectOption>{"Option 3"}</SelectOption>
                </Select>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Select with null"}</span>
                <SelectWithNull state={select_with_null_state.clone()} label="Select with null label" null_label="Select an option..." required={true} error={select_with_null_value.is_none().then_some("Please select a value")}>
                    <SelectOption>{"Option 1"}</SelectOption>
                    <SelectOption>{"Option 2"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 3 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 4 (disabled)"}</SelectOption>
                    <SelectOption>{"Option 5"}</SelectOption>
                </SelectWithNull>
                <span>{"Value: "}{select_with_null_value.map(|x| x.to_string()).unwrap_or("None".to_owned())}</span>
                <SelectWithNull state={select_with_null_state} label="Disabled select with null label" disabled={true}>
                    <SelectOption>{"Option 1"}</SelectOption>
                    <SelectOption>{"Option 2"}</SelectOption>
                    <SelectOption>{"Option 3"}</SelectOption>
                </SelectWithNull>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Dialog"}</span>
                <Button text="Open small dialog" on_click={move |_| dialog_small_button_state.set(true)} />
                <Dialog
                    state={dialog_small_state}
                    size={DialogSize::Small}
                    title="Small dialog"
                    ok_label="OK"
                    cancel_label="Cancel"
                    on_close={move |ok| dialog_small_close_state.set(Some(ok))}
                    actions_layout={DialogActionsLayout::Left}
                >
                    <p>{"A small dialog with left-aligned actions."}</p>
                </Dialog>
                <Button text="Open medium dialog" on_click={move |_| dialog_medium_button_state.set(true)} />
                <Dialog
                    state={dialog_medium_state}
                    size={DialogSize::Medium}
                    title="Medium dialog"
                    ok_label="OK"
                    cancel_label="Cancel"
                    on_close={move |ok| dialog_medium_close_state.set(Some(ok))}
                    actions_layout={DialogActionsLayout::Right}
                >
                    <p>{"A medium dialog with right-aligned actions."}</p>
                    <p>{"Test"}</p>
                    <p>{"Scrolling"}</p>
                    <p>{"Behavior"}</p>
                    <p>{"Test"}</p>
                    <p>{"Scrolling"}</p>
                    <p>{"Behavior"}</p>
                    <p>{"Test"}</p>
                    <p>{"Scrolling"}</p>
                    <p>{"Behavior"}</p>
                    <p>{"Test"}</p>
                    <p>{"Scrolling"}</p>
                    <p>{"Behavior"}</p>
                    <p>{"Test"}</p>
                    <p>{"Scrolling"}</p>
                    <p>{"Behavior"}</p>
                    <p>{"Test"}</p>
                    <p>{"Scrolling"}</p>
                    <p>{"Behavior"}</p>
                    <p>{"Test"}</p>
                    <p>{"Scrolling"}</p>
                    <p>{"Behavior"}</p>
                    <p>{"Test"}</p>
                    <p>{"Scrolling"}</p>
                    <p>{"Behavior"}</p>
                </Dialog>
                <Button text="Open large dialog" on_click={move |_| dialog_large_button_state.set(true)} />
                <Dialog
                    state={dialog_large_state}
                    size={DialogSize::Large}
                    title="Large dialog"
                    ok_label="OK"
                    cancel_label="Cancel"
                    on_close={move |ok| dialog_large_close_state.set(Some(ok))}
                    actions_layout={DialogActionsLayout::Spaced}
                >
                    <p>{"A large dialog with spaced actions."}</p>
                    <Select state={dialog_select_state} label="Dialog select label">
                        <SelectOption>{"Option 1"}</SelectOption>
                        <SelectOption>{"Option 2"}</SelectOption>
                        <SelectOption>{"Option 3"}</SelectOption>
                        <SelectOption>{"Option 4"}</SelectOption>
                        <SelectOption>{"Option 5"}</SelectOption>
                    </Select>
                </Dialog>
                <Button text="Open max dialog" on_click={move |_| dialog_max_button_state.set(true)} />
                <Dialog
                    state={dialog_max_state}
                    size={DialogSize::Max}
                    title="Max dialog"
                    on_close={move |ok| dialog_max_close_state.set(Some(ok))}
                >
                    <p>{"A maximum size dialog with no actions."}</p>
                </Dialog>
                <Button text="Open auto dialog" on_click={move |_| dialog_auto_button_state.set(true)} />
                <Dialog
                    state={dialog_auto_state}
                    size={DialogSize::Auto}
                    title="Auto dialog"
                    ok_label="OK"
                    on_close={move |ok| dialog_auto_close_state.set(Some(ok))}
                >
                    <p>{"An auto size dialog with only an OK action."}</p>
                </Dialog>
                <span>{"Close value: "}{(*dialog_close_state).map(|x| x.to_string()).unwrap_or("None".to_owned())}</span>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Alert"}</span>
                <Button text="Open 5 second alert" on_click={move |_| alert_finite_button_state.set(true)} />
                <Alert
                    state={alert_finite_state}
                    title="Finite alert"
                    duration={AlertDuration::Finite(5)}
                    on_close={move |manual| alert_finite_close_state.set(Some(manual))}
                >
                    <p>{"This alert will only remain open for 5 seconds."}</p>
                </Alert>
                <Button text="Open infinite alert" on_click={move |_| alert_infinite_button_state.set(true)} />
                <Alert
                    state={alert_infinite_state}
                    title="Infinite alert"
                    duration={AlertDuration::Infinite}
                    on_close={move |manual| alert_infinite_close_state.set(Some(manual))}
                >
                    <p>{"This alert will remain open until the 'x' button is pressed."}</p>
                </Alert>
                <span>{"Close value: "}{(*alert_close_state).map(|x| x.to_string()).unwrap_or("None".to_owned())}</span>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Card"}</span>
                <Card
                    interactive={true}
                    on_click={move |_| card_interactive_state.set(Some(1))}
                >
                    <h3>{"Interactive card"}</h3>
                    <p>{"Notice the zoom animation and pointer cursor when hovering."}</p>
                </Card>
                <Card on_click={move |_| card_not_interactive_state.set(Some(2))}>
                    <h3>{"Not an interactive card"}</h3>
                    <p>{"No hover animation or pointer cursor on this one."}</p>
                </Card>
                <span>{"Card click state: "}{(*card_state).map(|x| x.to_string()).unwrap_or("None".to_owned())}</span>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Progress bar"}</span>
                <ProgressBar progress={0.0} />
                <ProgressBar progress={0.05} />
                <ProgressBar progress={0.2} />
                <ProgressBar progress={0.5} />
                <ProgressBar progress={0.8} />
                <ProgressBar progress={0.95} />
                <ProgressBar progress={1.0} />
                <ProgressBar progress={0.5} disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Spinner"}</span>
                <Spinner size={SpinnerSize::Small} center={false} />
                <Spinner size={SpinnerSize::Medium} center={false} />
                <Spinner size={SpinnerSize::Large} center={false} />
                <Spinner size={SpinnerSize::Max} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Chips"}</span>
                <Chips
                    state={chips_state.clone()}
                    options={chip_options.clone()}
                    option_limit={5}
                    label="Chips label"
                    placeholder="Placeholder!"
                    error={chips_value.is_empty().then_some("Please select at least one language")}
                />
                <Chips
                    state={chips_state}
                    options={chip_options}
                    label="Disabled chips label"
                    disabled={true}
                />
                <span>{"Selected: "}{chips_value.join(", ")}</span>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Tooltip"}</span>
                <Tooltip text="Tooltip hover text">{"Hover here to view the tooltip text"}</Tooltip>
                <Tooltip text="This should not show" disabled={true}>{"This tooltip is disabled, and should show nothing when hovered over"}</Tooltip>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Badge"}</span>
                <div class="base-demo-item-row">
                    {"Primary badge"}
                    <Badge value={3} style={BadgeStyle::Primary} />
                </div>
                <div class="base-demo-item-row">
                    {"Secondary badge"}
                    <Badge value={42} style={BadgeStyle::Secondary} />
                </div>
                <div class="base-demo-item-row">
                    {"Danger badge"}
                    <Badge<f64> value={1.618} style={BadgeStyle::Danger} />
                </div>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Date picker"}</span>
                <DatePicker state={datepicker_state.clone()} label="Date picker label" min={date_min} max={date_max} required={true} error={date_error} />
                <span>{"Selected date: "}{datepicker_value.map(|x| x.to_string()).unwrap_or("None".to_owned())}</span>
                <DatePicker state={datepicker_state} label="Disabled date picker label" disabled={true} />
            </div>
        </div>
    }
}
