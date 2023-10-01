use crate::components::base::*;
use crate::hooks::*;
use crate::view::*;
use yew::prelude::*;

/// The root element of the application.
#[function_component]
pub fn App() -> Html {
    let (view, dispatch) = use_view();
    let view_select_state = use_state(|| 0);
    let on_change = move |idx| {
        dispatch.set(match idx {
            0 => View::Home,
            1 => View::Settings,
            2 => View::Save,
            3 => View::Report,
            4 => View::ReportTemplate,
            5 => View::Search,
            _ => unreachable!(),
        })
    };

    html! {
        <div class="app">
            <div class="header">
                <p>{"Header placeholder"}</p>
            </div>
            <div class="main">
                <div style="display: flex;">
                    <Select
                        state={view_select_state}
                        {on_change}
                        label="Select view"
                    >
                        <SelectOption>{"Home"}</SelectOption>
                        <SelectOption>{"Settings"}</SelectOption>
                        <SelectOption>{"Save"}</SelectOption>
                        <SelectOption>{"Report"}</SelectOption>
                        <SelectOption>{"Report template"}</SelectOption>
                        <SelectOption>{"Search"}</SelectOption>
                    </Select>
                </div>
                {view.html_view()}
            </div>
            <div class="footer">
                <p>{"Footer placeholder"}</p>
            </div>
        </div>
    }
}
