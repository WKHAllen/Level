use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::view::View;
use commands::FrontendCommands;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// The page view to create a save.
#[function_component]
pub fn Create() -> Html {
    let view = use_view();

    let save_name_state = use_state(String::new);
    let save_description_state = use_state(String::new);
    let save_password_state = use_state(String::new);
    let save_password_confirm_state = use_state(String::new);
    let create_save_error_state = use_state(|| None);
    let loading_overlay_state = use_state(|| false);

    let save_name_focus = use_focus();
    use_mount({
        let save_name_focus = save_name_focus.clone();
        move || {
            save_name_focus.focus();
        }
    });

    let create_save_error = (*create_save_error_state).clone();

    let try_create_save = use_command(
        UseCommand::new({
            let save_name_state = save_name_state.clone();
            let save_description_state = save_description_state.clone();
            let save_password_state = save_password_state.clone();
            let save_password_confirm_state = save_password_confirm_state.clone();
            |backend| async move {
                let save_name = (*save_name_state).clone();
                let save_description = (*save_description_state).clone();
                let save_password = (*save_password_state).clone();
                let save_password_confirm = (*save_password_confirm_state).clone();

                if save_password == save_password_confirm {
                    backend
                        .create_save_file(save_name, save_description, save_password)
                        .await?;
                    Ok(Ok(()))
                } else {
                    Ok(Err("Error: passwords do not match"))
                }
            }
        })
        .run_on_init(false)
        .on_update({
            let view = view.clone();
            let create_save_error_state = create_save_error_state.clone();
            let loading_overlay_state = loading_overlay_state.clone();
            move |create_save_result| match create_save_result {
                UseCommandState::Init => {
                    loading_overlay_state.set(false);
                }
                UseCommandState::Loading => {
                    loading_overlay_state.set(true);
                    create_save_error_state.set(None);
                }
                UseCommandState::Resolved(res) => match res {
                    Ok(inner_res) => match inner_res {
                        Ok(_) => {
                            loading_overlay_state.set(false);
                            create_save_error_state.set(None);
                            view.set(View::Save);
                        }
                        Err(err) => {
                            loading_overlay_state.set(false);
                            create_save_error_state.set(Some((*err).to_owned()));
                        }
                    },
                    Err(err) => {
                        loading_overlay_state.set(false);
                        create_save_error_state.set(Some(err.to_string()));
                    }
                },
            }
        }),
    );

    let run_try_create_save = move |_| try_create_save.run();
    let go_back = move |_| view.set(View::Open);

    html! {
        <div class="view create">
            <h2>{"Create a new save file"}</h2>
            <div class="create-save-form">
                <Input
                    state={save_name_state}
                    label="Save name"
                    max_length={255}
                    on_submit={run_try_create_save.clone()}
                    required={true}
                    node={save_name_focus.node_ref()}
                />
                <TextArea
                    state={save_description_state}
                    label="Save description"
                    max_length={1023}
                    rows={4}
                />
                <div class="create-save-passwords">
                    <Input
                        state={save_password_state}
                        input_type={InputType::Password}
                        label="Save password"
                        max_length={255}
                        on_submit={run_try_create_save.clone()}
                        required={true}
                    />
                    <Input
                        state={save_password_confirm_state}
                        input_type={InputType::Password}
                        label="Confirm save password"
                        max_length={255}
                        on_submit={run_try_create_save.clone()}
                        required={true}
                    />
                </div>
                <div class="create-save-error">
                    <Error message={create_save_error} size={ErrorSize::Small} />
                </div>
                <div class="create-save-actions">
                    <Button
                        text="Create"
                        on_click={run_try_create_save}
                    />
                    <Button
                        text="Back"
                        on_click={go_back}
                        style={ButtonStyle::Secondary}
                    />
                </div>
            </div>
            <LoadingOverlay state={loading_overlay_state} />
        </div>
    }
}
