use super::*;
use crate::hooks::*;
use crate::util::*;
use chrono::{Datelike, Duration, Local, NaiveDate};
use std::borrow::Borrow;
use std::ops::Deref;
use yew::prelude::*;
use yew_hooks::use_click_away;

/// Month names.
const MONTHS: &[&str; 12] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Returns the string representation of a year.
fn year_to_string(year: i32) -> String {
    format!("{:0>4}", year.to_string())
}

/// Returns the string representation of a month.
fn month_to_string(month: u32) -> String {
    format!("{:0>2}", month.to_string())
}

/// Returns the string representation of a day.
fn day_to_string(day: u32) -> String {
    format!("{:0>2}", day.to_string())
}

/// Determines the new year string value.
fn new_year_value(old_year: &str, new_year: &str) -> String {
    if new_year.is_empty() {
        return year_to_string(Default::default());
    }

    match new_year.parse::<i32>() {
        Ok(parsed_year) => {
            let year_str = year_to_string(parsed_year);

            if year_str.len() <= 4 {
                year_str
            } else {
                (year_str[year_str.len() - 4..]).to_owned()
            }
        }
        Err(_) => old_year.to_owned(),
    }
}

/// Determines the new month string value.
fn new_month_value(old_month: &str, new_month: &str) -> String {
    if new_month.is_empty() {
        return month_to_string(Default::default());
    }

    match new_month.parse::<u32>() {
        Ok(parsed_month) => {
            let month_str = month_to_string(parsed_month);

            if month_str.len() <= 2 {
                month_str
            } else {
                (month_str[month_str.len() - 2..]).to_owned()
            }
        }
        Err(_) => old_month.to_owned(),
    }
}

/// Determines the new day string value.
fn new_day_value(old_day: &str, new_day: &str) -> String {
    if new_day.is_empty() {
        return day_to_string(Default::default());
    }

    match new_day.parse::<u32>() {
        Ok(parsed_day) => {
            let day_str = day_to_string(parsed_day);

            if day_str.len() <= 2 {
                day_str
            } else {
                (day_str[day_str.len() - 2..]).to_owned()
            }
        }
        Err(_) => old_day.to_owned(),
    }
}

/// Attempts to parse the provided date value into a `NaiveDate`. Fails with
/// the error message if parsing fails.
fn parse_date(year_str: &str, month_str: &str, day_str: &str) -> Result<NaiveDate, String> {
    let year = year_str
        .parse::<i32>()
        .map_err(|_| "Invalid year".to_owned())?;
    let month = month_str
        .parse::<u32>()
        .map_err(|_| "Invalid month".to_owned())?;
    let day = day_str
        .parse::<u32>()
        .map_err(|_| "Invalid day".to_owned())?;

    NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date".to_owned())
}

/// Validates that the provided date falls within the given range.
fn date_within_range(date: NaiveDate, min: NaiveDate, max: NaiveDate) -> bool {
    min <= date && date <= max
}

/// Checks the validity of the date state.
fn check_state(
    year_str: &str,
    month_str: &str,
    day_str: &str,
    min: NaiveDate,
    max: NaiveDate,
) -> Result<NaiveDate, String> {
    let date = parse_date(year_str, month_str, day_str)?;

    if date_within_range(date, min, max) {
        Ok(date)
    } else {
        Err(format!("Date must be between {min} and {max}"))
    }
}

/// Returns the name of the month.
fn month_name(date: NaiveDate) -> String {
    MONTHS[date.month0() as usize].to_owned()
}

/// Determines the previous month.
fn prev_month(date: NaiveDate) -> NaiveDate {
    if date.month() == 1 {
        date.with_year(date.year() - 1)
            .unwrap()
            .with_month(12)
            .unwrap()
            .with_day(1)
            .unwrap()
    } else {
        date.with_month(date.month() - 1)
            .unwrap()
            .with_day(1)
            .unwrap()
    }
}

/// Determines the next month.
fn next_month(date: NaiveDate) -> NaiveDate {
    if date.month() == 12 {
        date.with_year(date.year() + 1)
            .unwrap()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
    } else {
        date.with_month(date.month() + 1)
            .unwrap()
            .with_day(1)
            .unwrap()
    }
}

/// Determines whether the previous month on the calendar is viewable.
fn prev_month_viewable(viewing_month: NaiveDate, min: NaiveDate) -> bool {
    min.with_day(1).unwrap() < viewing_month.with_day(1).unwrap()
}

/// Determines whether the next month on the calendar is viewable.
fn next_month_viewable(viewing_month: NaiveDate, max: NaiveDate) -> bool {
    viewing_month.with_day(1).unwrap() < max.with_day(1).unwrap()
}

/// Determines how many days need to be displayed before the start of the
/// currently viewed month.
fn days_before_month(viewing_month: NaiveDate) -> u32 {
    let first_of_month = viewing_month.with_day(1).unwrap();
    first_of_month.weekday().num_days_from_sunday()
}

/// Determines how many days need to be displayed within the currently viewed
/// month.
fn days_in_month(viewing_month: NaiveDate) -> u32 {
    let last_of_month = next_month(viewing_month) - Duration::days(1);
    last_of_month.day()
}

/// Determines how many days need to be displayed after the end of the
/// currently viewed month.
fn days_after_month(viewing_month: NaiveDate) -> u32 {
    let calendar_space = 42;
    let num_days_before_month = days_before_month(viewing_month);
    let num_days_in_month = days_in_month(viewing_month);
    calendar_space - num_days_in_month - num_days_before_month
}

/// Gets the calendar day of the currently viewed month with the specified day.
fn calendar_day(viewing_month: NaiveDate, day: u32) -> Option<NaiveDate> {
    viewing_month.with_day(day)
}

/// Returns a `NaiveDate` representing the current local date. This is provided
/// for use with the `DatePicker` component.
pub fn date_picker_today() -> NaiveDate {
    Local::now().naive_local().date()
}

/// A wrapper around a date picker state.
#[derive(Debug, Clone, PartialEq)]
pub struct DatePickerState {
    /// The inner state.
    state: Option<NaiveDate>,
    /// The year input.
    year: String,
    /// The month input.
    month: String,
    /// The day input.
    day: String,
}

#[allow(dead_code)]
impl DatePickerState {
    /// Creates a new date picker state with no date selected.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new date picker state with the given date selected.
    pub fn new_with(state: NaiveDate) -> Self {
        let mut this = Self::new();
        this.state = Some(state);
        this.year = year_to_string(state.year());
        this.month = month_to_string(state.month());
        this.day = day_to_string(state.day());
        this
    }

    /// Creates a new date picker state with today selected.
    pub fn new_today() -> Self {
        Self::new_with(date_picker_today())
    }

    /// Updates the inner state, if necessary.
    fn update(
        &mut self,
        year: Option<&str>,
        month: Option<&str>,
        day: Option<&str>,
        min: NaiveDate,
        max: NaiveDate,
    ) {
        self.state = check_state(
            year.unwrap_or(&self.year),
            month.unwrap_or(&self.month),
            day.unwrap_or(&self.day),
            min,
            max,
        )
        .ok();

        if let Some(year) = year {
            self.year = year.to_owned();
        }

        if let Some(month) = month {
            self.month = month.to_owned();
        }

        if let Some(day) = day {
            self.day = day.to_owned();
        }
    }

    /// Sets the inner state.
    fn set(&mut self, date: NaiveDate) {
        self.state = Some(date);
        self.year = year_to_string(date.year());
        self.month = month_to_string(date.month());
        self.day = day_to_string(date.day());
    }
}

impl Deref for DatePickerState {
    type Target = Option<NaiveDate>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl Borrow<Option<NaiveDate>> for DatePickerState {
    fn borrow(&self) -> &Option<NaiveDate> {
        &self.state
    }
}

impl Default for DatePickerState {
    fn default() -> Self {
        let today = date_picker_today();

        Self {
            state: None,
            year: year_to_string(today.year()),
            month: month_to_string(today.month()),
            day: day_to_string(today.day()),
        }
    }
}

/// Position of a date picker popup.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DatePickerPopupPosition {
    /// Position the popup above.
    Above,
    /// Position the popup below.
    #[default]
    Below,
}

impl DatePickerPopupPosition {
    /// Gets the name of the position.
    pub fn position_name(&self) -> &'static str {
        match *self {
            Self::Above => "above",
            Self::Below => "below",
        }
    }
}

/// Date picker properties.
#[derive(Properties, PartialEq, Clone)]
pub struct DatePickerProps {
    /// The date picker state.
    pub state: UseStateHandle<DatePickerState>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<Option<NaiveDate>>,
    /// The date picker label.
    #[prop_or_default]
    pub label: AttrValue,
    /// The earliest date to allow.
    #[prop_or(NaiveDate::from_ymd_opt(0000, 1, 1).unwrap())]
    pub min: NaiveDate,
    /// The latest date to allow.
    #[prop_or(NaiveDate::from_ymd_opt(9999, 12, 31).unwrap())]
    pub max: NaiveDate,
    /// The positioning of the popup.
    #[prop_or_default]
    pub position: DatePickerPopupPosition,
    /// Whether a date must be picked.
    #[prop_or(false)]
    pub required: bool,
    /// Whether to compact the element into a smaller space.
    #[prop_or(false)]
    pub compact: bool,
    /// An optional error message. This may not show at times, as it will be
    /// overridden by a different error message if validation fails.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the date picker is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The year input node ref.
    #[prop_or_default]
    pub year_node: NodeRef,
    /// The month input node ref.
    #[prop_or_default]
    pub month_node: NodeRef,
    /// The day input node ref.
    #[prop_or_default]
    pub day_node: NodeRef,
}

/// A date picker component.
#[function_component]
pub fn DatePicker(props: &DatePickerProps) -> Html {
    let DatePickerProps {
        state,
        on_change,
        label,
        min,
        max,
        position,
        required,
        compact,
        error,
        disabled,
        year_node,
        month_node,
        day_node,
    } = props.clone();

    use_effect_with(state.clone(), move |new_state| on_change.emit(***new_state));

    let year_id_state = use_id();
    let year_id = (*year_id_state).clone();
    let month_id_state = use_id();
    let month_id = (*month_id_state).clone();
    let day_id_state = use_id();
    let day_id = (*day_id_state).clone();
    let calendar_open = use_state(|| false);
    let today = date_picker_today();
    let year_value = state.year.clone();
    let month_value = state.month.clone();
    let day_value = state.day.clone();
    let viewing_calendar_month_state = use_state(|| date_picker_today().with_day(1).unwrap());
    let viewing_calendar_month_name = format!(
        "{} {}",
        month_name(*viewing_calendar_month_state),
        (*viewing_calendar_month_state).year()
    );
    let prev_month_disabled = !prev_month_viewable(*viewing_calendar_month_state, min);
    let next_month_disabled = !next_month_viewable(*viewing_calendar_month_state, max);
    let num_days_before_month = days_before_month(*viewing_calendar_month_state);
    let num_days_in_month = days_in_month(*viewing_calendar_month_state);
    let num_days_after_month = days_after_month(*viewing_calendar_month_state);

    let error_msg = check_state(&year_value, &month_value, &day_value, min, max)
        .err()
        .map(|err| err.into())
        .or(error);

    let year_on_focus_in = {
        let year_node = year_node.clone();
        move |_| {
            select_element_content(&year_node);
        }
    };
    let month_on_focus_in = {
        let month_node = month_node.clone();
        move |_| {
            select_element_content(&month_node);
        }
    };
    let day_on_focus_in = {
        let day_node = day_node.clone();
        move |_| {
            select_element_content(&day_node);
        }
    };

    let year_on_input = {
        let state = state.clone();
        let year_node = year_node.clone();
        move |event: InputEvent| {
            let new_typed_value = content_editable_event_value(event);
            let new_value = new_year_value(&state.year, &new_typed_value);
            set_inner_text(&year_node, &new_value);
            go_to_end(&year_node);
            let mut new_state = (*state).clone();
            new_state.update(Some(&new_value), None, None, min, max);
            state.set(new_state);
        }
    };
    let month_on_input = {
        let state = state.clone();
        let month_node = month_node.clone();
        move |event: InputEvent| {
            let new_typed_value = content_editable_event_value(event);
            let new_value = new_month_value(&state.month, &new_typed_value);
            set_inner_text(&month_node, &new_value);
            go_to_end(&month_node);
            let mut new_state = (*state).clone();
            new_state.update(None, Some(&new_value), None, min, max);
            state.set(new_state);
        }
    };
    let day_on_input = {
        let state = state.clone();
        let day_node = day_node.clone();
        move |event: InputEvent| {
            let new_typed_value = content_editable_event_value(event);
            let new_value = new_day_value(&state.day, &new_typed_value);
            set_inner_text(&day_node, &new_value);
            go_to_end(&day_node);
            let mut new_state = (*state).clone();
            new_state.update(None, None, Some(&new_value), min, max);
            state.set(new_state);
        }
    };

    let on_calendar_focus_in = |_| clear_selections();
    let on_calendar_button_click = {
        let calendar_open = calendar_open.clone();
        move |_| {
            calendar_open.set(true);
        }
    };

    let popup_node = use_node_ref();
    use_click_away(popup_node.clone(), {
        let calendar_open = calendar_open.clone();
        move |_| {
            calendar_open.set(false);
        }
    });
    use_popup(popup_node.clone());

    let on_prev_month_click = {
        let viewing_calendar_month_state = viewing_calendar_month_state.clone();
        move |_| {
            viewing_calendar_month_state.set(prev_month(*viewing_calendar_month_state));
        }
    };
    let on_next_month_click = {
        let viewing_calendar_month_state = viewing_calendar_month_state.clone();
        move |_| {
            viewing_calendar_month_state.set(next_month(*viewing_calendar_month_state));
        }
    };

    let calendar_days_prev = (0..num_days_before_month)
        .map(|_| {
            html! {
                <div class="base-date-picker-calendar-day base-date-picker-calendar-day-hidden"></div>
            }
        })
        .collect::<Html>();
    let calendar_days_current = (1..=num_days_in_month)
        .map(|i| {
            let this_day = calendar_day(*viewing_calendar_month_state, i).unwrap();
            let day_selected = **state == Some(this_day);
            let day_today = this_day == today;
            let day_disabled = !date_within_range(this_day, min, max);

            let day_on_click = {
                let year_node = year_node.clone();
                let month_node = month_node.clone();
                let day_node = day_node.clone();
                let state = state.clone();
                let calendar_open = calendar_open.clone();
                move |_| {
                    let mut new_state = (*state).clone();
                    new_state.set(this_day);
                    state.set(new_state);
                    calendar_open.set(false);
                    let year_str = year_to_string(this_day.year());
                    let month_str = month_to_string(this_day.month());
                    let day_str = day_to_string(this_day.day());
                    set_inner_text(&year_node, &year_str);
                    set_inner_text(&month_node, &month_str);
                    set_inner_text(&day_node, &day_str);
                }
            };

            html! {
                <div class={classes!("base-date-picker-calendar-day", day_selected.then_some("base-date-picker-calendar-day-selected"), day_today.then_some("base-date-picker-calendar-day-today"), day_disabled.then_some("base-date-picker-calendar-day-disabled"))}>
                    <button
                        type="button"
                        onclick={day_on_click}
                        disabled={day_disabled}
                        class="base-date-picker-calendar-day-button"
                    >
                        <div class="base-date-picker-calendar-day-button-text">{i}</div>
                    </button>
                </div>
            }
        })
        .collect::<Html>();
    let calendar_days_next = (0..num_days_after_month)
        .map(|_| {
            html! {
                <div class="base-date-picker-calendar-day base-date-picker-calendar-day-hidden"></div>
            }
        })
        .collect::<Html>();

    let position_class = format!("base-date-picker-{}", position.position_name());

    html! {
        <div class={classes!("base-date-picker-container", compact.then_some("base-date-picker-container-compact"), disabled.then_some("base-date-picker-container-disabled"))}>
            <label for={year_id.clone()} class="base-date-picker-label">
                {label}
                <span class="base-required-mark">{required.then_some(" *").unwrap_or_default()}</span>
            </label>
            <div class={classes!("base-date-picker-outer", position_class)}>
                <div class={classes!("base-date-picker", error_msg.as_ref().map(|_| "base-date-picker-invalid"))}>
                    <div class="base-date-picker-section">
                        <span
                            ref={year_node}
                            id={year_id}
                            contenteditable={(!disabled).to_string()}
                            onfocusin={year_on_focus_in}
                            oninput={year_on_input}
                            class="base-date-picker-input"
                        >{year_value}</span>
                        <span>{"-"}</span>
                        <span
                            ref={month_node}
                            id={month_id}
                            contenteditable={(!disabled).to_string()}
                            onfocusin={month_on_focus_in}
                            oninput={month_on_input}
                            class="base-date-picker-input"
                        >{month_value}</span>
                        <span>{"-"}</span>
                        <span
                            ref={day_node}
                            id={day_id}
                            contenteditable={(!disabled).to_string()}
                            onfocusin={day_on_focus_in}
                            oninput={day_on_input}
                            class="base-date-picker-input"
                        >{day_value}</span>
                    </div>
                    <div class="base-date-picker-section" onfocusin={on_calendar_focus_in}>
                        <IconButton name="calendar-days-solid" size={IconButtonSize::Medium} {disabled} on_click={on_calendar_button_click} />
                    </div>
                </div>
                <div class={classes!("base-date-picker-popup-container-outer", (*calendar_open).then_some("base-date-picker-popup-container-open"))}>
                    <div class="base-date-picker-popup-container-inner">
                        <div ref={popup_node} class="base-date-picker-popup">
                            <div class="base-date-picker-calendar">
                                <div class="base-date-picker-calendar-month-controls">
                                    <IconButton
                                        name="angle-left-solid"
                                        size={IconButtonSize::Medium}
                                        on_click={on_prev_month_click}
                                        disabled={prev_month_disabled}
                                    />
                                    <span class="base-date-picker-calendar-month">{viewing_calendar_month_name}</span>
                                    <IconButton
                                        name="angle-right-solid"
                                        size={IconButtonSize::Medium}
                                        on_click={on_next_month_click}
                                        disabled={next_month_disabled}
                                    />
                                </div>
                                <div class="base-date-picker-calendar-days-of-week">
                                    <span>{"Su"}</span>
                                    <span>{"Mo"}</span>
                                    <span>{"Tu"}</span>
                                    <span>{"We"}</span>
                                    <span>{"Th"}</span>
                                    <span>{"Fr"}</span>
                                    <span>{"Sa"}</span>
                                </div>
                                <div class="base-date-picker-calendar-view">
                                    {calendar_days_prev}
                                    {calendar_days_current}
                                    {calendar_days_next}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <Error message={error_msg} size={ErrorSize::Small} class="base-date-picker-error" />
        </div>
    }
}
