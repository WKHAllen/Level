use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::util::*;
use crate::validation::*;
use crate::view::View;
use commands::FrontendCommands;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// The page view to create a save.
#[function_component]
pub fn Create() -> Html {
    let view = use_view();

    let save_name_state = use_state(String::new);
    let save_name_error_state = use_state(|| None);
    let save_description_state = use_state(String::new);
    let save_description_error_state = use_state(|| None);
    let save_password_state = use_state(String::new);
    let save_password_error_state = use_state(|| None);
    let save_password_confirm_state = use_state(String::new);
    let create_save_error_state = use_state(|| None);
    let loading_overlay_state = use_state(|| false);

    let save_name_node = use_node_ref();
    let save_name_focus = use_focus(save_name_node);
    use_mount({
        let save_name_focus = save_name_focus.clone();
        move || {
            save_name_focus.focus();
        }
    });

    let create_save_error = (*create_save_error_state).clone();

    let try_create_save = use_command(
        UseCommand::new({
            clone_states!(
                save_name_state,
                save_name_error_state,
                save_description_state,
                save_description_error_state,
                save_password_state,
                save_password_error_state,
                save_password_confirm_state,
            );
            |backend| async move {
                if let Some((name, description, password)) = validate_all!(
                    validate(save_name_state, save_name_error_state, validate_save_name),
                    validate(
                        save_description_state,
                        save_description_error_state,
                        validate_save_description
                    ),
                    validate_with(
                        save_password_state,
                        save_password_error_state,
                        validate_save_password,
                        &*save_password_confirm_state
                    )
                ) {
                    backend
                        .create_save_file(name, description, password)
                        .await?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        })
        .run_on_init(false)
        .on_update({
            clone_states!(view, create_save_error_state, loading_overlay_state);
            move |create_save_result| match create_save_result {
                UseCommandState::Init => {
                    loading_overlay_state.set(false);
                }
                UseCommandState::Loading => {
                    loading_overlay_state.set(true);
                    create_save_error_state.set(None);
                }
                UseCommandState::Resolved(res) => match res {
                    Ok(success) => {
                        if *success {
                            loading_overlay_state.set(false);
                            create_save_error_state.set(None);
                            view.set(View::Save);
                        }
                    }
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
                    error={(*save_name_error_state).clone()}
                />
                <TextArea
                    state={save_description_state}
                    label="Save description"
                    max_length={1023}
                    rows={4}
                    error={(*save_description_error_state).clone()}
                />
                <div class="create-save-passwords">
                    <Input
                        state={save_password_state}
                        input_type={InputType::Password}
                        label="Save password"
                        max_length={255}
                        on_submit={run_try_create_save.clone()}
                        required={true}
                        error={(*save_password_error_state).clone()}
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
