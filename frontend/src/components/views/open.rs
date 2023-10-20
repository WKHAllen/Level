use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::util::*;
use crate::view::View;
use chrono::prelude::*;
use commands::FrontendCommands;
use common::*;
use js_sys::{Date, Object};
use wasm_bindgen::JsValue;
use yew::prelude::*;

/// Gets the user locale from the browser.
fn user_locale() -> String {
    let window = window();
    let navigator = window.navigator();
    let languages = navigator.languages();
    let language = navigator.language();
    let first_language = languages.get(0).as_string();

    if let Some(lang) = first_language {
        lang
    } else if let Some(lang) = language {
        lang
    } else {
        "en-US".to_owned()
    }
}

/// Formats a timestamp using the JS Date API's `.toLocaleString()` method.
fn locale_timestamp_str(timestamp: &NaiveDateTime) -> String {
    let datetime = timestamp.and_local_timezone(Utc).unwrap();
    let js_millis = JsValue::from_f64(datetime.timestamp_millis() as f64);
    let js_date = Date::new(&js_millis);
    let locale = user_locale();
    let options = Object::new();
    js_date
        .to_locale_string(&locale, &options)
        .as_string()
        .unwrap()
}

/// The page view to open a save.
#[function_component]
pub fn Open() -> Html {
    let view = use_view();

    let saves = use_command(UseCommand::new(|backend| async move {
        backend.list_save_files().await
    }));

    let dialog_open_state = use_state(|| false);
    let selected_save_state = use_state(|| None::<SaveMetadata>);
    let save_password_state = use_state(String::new);
    let unlock_save_error_state = use_state(|| None);
    let loading_overlay_state = use_state(|| false);

    let save_password = (*save_password_state).clone();
    let unlock_save_error = (*unlock_save_error_state).clone();

    let default_timestamp = NaiveDate::from_ymd_opt(0000, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let try_open_save = use_command(
        UseCommand::new({
            let selected_save_state = selected_save_state.clone();
            move |backend| async move {
                let save_name = selected_save_state
                    .as_ref()
                    .map(|save| save.name.clone())
                    .unwrap_or_else(|| "<SAVE NAME>".to_owned());
                backend.open_save_file(save_name, save_password).await
            }
        })
        .run_on_init(false)
        .on_update({
            let loading_overlay_state = loading_overlay_state.clone();
            let unlock_save_error_state = unlock_save_error_state.clone();
            move |open_save_result| match open_save_result {
                UseCommandState::Init => {
                    loading_overlay_state.set(false);
                }
                UseCommandState::Loading => {
                    loading_overlay_state.set(true);
                    unlock_save_error_state.set(None);
                }
                UseCommandState::Resolved(res) => match res {
                    Ok(_) => {
                        loading_overlay_state.set(false);
                        unlock_save_error_state.set(None);
                        view.set(View::Save);
                    }
                    Err(err) => {
                        loading_overlay_state.set(false);
                        unlock_save_error_state.set(Some(err.to_string()));
                    }
                },
            }
        }),
    );

    match &*saves {
        UseCommandState::Init | UseCommandState::Loading => html! { <Loading /> },
        UseCommandState::Resolved(saves) => match saves {
            Err(_err) => todo!("handle errors correctly"),
            Ok(saves) => {
                let mut saves = saves.clone();
                saves.sort_by_key(|save| save.last_opened_at);
                saves.reverse();

                let save_buttons = saves
                    .into_iter()
                    .map(|save| {
                        let save_name = save.name.clone();
                        let onclick = {
                            let dialog_open_state = dialog_open_state.clone();
                            let selected_save_state = selected_save_state.clone();
                            move |_| {
                                selected_save_state.set(Some(save.clone()));
                                dialog_open_state.set(true);
                            }
                        };

                        html! {
                            <div class="open-save">
                                <div class="open-save-button hoverable" {onclick}>
                                    <SaveIcon open={false} size={48} />
                                    <span class="open-save-name clamp-3">{save_name}</span>
                                </div>
                            </div>
                        }
                    })
                    .collect::<Html>();

                let (save_name, save_description, save_created_at, save_last_opened_at) =
                    selected_save_state
                        .as_ref()
                        .map(|save| {
                            (
                                save.name.clone(),
                                save.description.clone(),
                                save.created_at,
                                save.last_opened_at,
                            )
                        })
                        .unwrap_or_else(|| {
                            (
                                "<SAVE NAME>".to_owned(),
                                "<SAVE DESCRIPTION>".to_owned(),
                                default_timestamp,
                                default_timestamp,
                            )
                        });

                let created_at_str = locale_timestamp_str(&save_created_at);
                let last_opened_at_str = locale_timestamp_str(&save_last_opened_at);

                let input_open_save = move |_| try_open_save.run();
                let dialog_open_save = {
                    let input_open_save = input_open_save.clone();
                    move |unlock| {
                        if unlock {
                            input_open_save(());
                        }
                    }
                };

                html! {
                    <div class="view open">
                        <h2>{"Open a save file"}</h2>
                        <Frame>
                            <div class="open-saves">
                                {save_buttons}
                            </div>
                        </Frame>
                        <Dialog
                            state={dialog_open_state}
                            title={save_name}
                            ok_label="Unlock"
                            cancel_label="Close"
                            on_close={dialog_open_save}
                        >
                            <p>{save_description}</p>
                            <p>{"Created at: "}{created_at_str}</p>
                            <p>{"Last opened: "}{last_opened_at_str}</p>
                            <p>{"The save file can be unlocked by providing the save's password below."}</p>
                            <Input
                                state={save_password_state}
                                input_type={InputType::Password}
                                label="Password"
                                on_submit={input_open_save}
                                error={unlock_save_error}
                            />
                        </Dialog>
                        <LoadingOverlay state={loading_overlay_state} />
                    </div>
                }
            }
        },
    }
}
