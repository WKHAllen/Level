use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::util::*;
use crate::validation::*;
use commands::FrontendCommands;
use common::*;
use yew::prelude::*;

/// Institution configuration subview properties.
#[derive(Clone, PartialEq, Properties)]
pub struct EditInstitutionsProps {
    /// The callback called when the subview is exited.
    #[prop_or_default]
    pub on_exit: Callback<()>,
}

/// The institution editing subview.
#[function_component]
pub fn EditInstitutions(props: &EditInstitutionsProps) -> Html {
    let EditInstitutionsProps { on_exit } = props.clone();

    let institution_state = use_state(|| None);
    let institution_options_state = use_state(Vec::new);
    let institution_name_state = use_state(String::new);
    let institution_name_error_state = use_state(|| None::<String>);
    let institution_description_state = use_state(String::new);
    let institution_description_error_state = use_state(|| None::<String>);
    let new_institution_state = use_state(|| None::<Institution>);
    let loading_state = use_state(|| false);

    let subview = use_subview();

    let institution_options = institution_options_state
        .iter()
        .map(|option: &Institution| option.name.clone())
        .collect::<Vec<_>>();

    let get_institutions = use_command(UseCommand::new({
        clone_states!(
            institution_state,
            institution_options_state,
            new_institution_state
        );
        |backend| async move {
            let institutions = backend.institutions().await?;

            if let Some(new_institution) = &*new_institution_state {
                if let Some(index) = institutions
                    .iter()
                    .position(|institution| institution.id == new_institution.id)
                {
                    institution_state.set(Some(index));
                }

                new_institution_state.set(None);
            }

            institution_options_state.set(institutions);
            Ok(())
        }
    }));

    let create_institution = use_command(
        UseCommand::new({
            clone_states!(
                institution_name_state,
                institution_name_error_state,
                institution_description_state,
                institution_description_error_state,
            );
            |backend| async move {
                if let Some((name, description)) = validate_all!(
                    validate(
                        institution_name_state,
                        institution_name_error_state,
                        validate_institution_name
                    ),
                    validate(
                        institution_description_state,
                        institution_description_error_state,
                        validate_institution_description
                    )
                ) {
                    backend
                        .create_institution(name, description)
                        .await
                        .map(Some)
                } else {
                    Ok(None)
                }
            }
        })
        .run_on_init(false)
        .on_update({
            clone_states!(
                institution_name_state,
                institution_description_state,
                new_institution_state,
                loading_state,
                get_institutions
            );
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);

                    // TODO: handle future expected errors, e.g. duplicate institution name
                    if let Ok(Some(institution)) = res {
                        new_institution_state.set(Some(institution.clone()));
                        institution_name_state.set(String::new());
                        institution_description_state.set(String::new());
                    }

                    get_institutions.run();
                }
            }
        }),
    );

    let edit_institution = use_command(
        UseCommand::new({
            clone_states!(
                institution_state,
                institution_options_state,
                institution_name_state,
                institution_name_error_state,
                institution_description_state,
                institution_description_error_state,
            );
            |backend| async move {
                if let Some((name, description)) = validate_all!(
                    validate(
                        institution_name_state,
                        institution_name_error_state,
                        validate_institution_name
                    ),
                    validate(
                        institution_description_state,
                        institution_description_error_state,
                        validate_institution_description
                    )
                ) {
                    if let Some(index) = &*institution_state {
                        if let Some(institution) = institution_options_state.get(*index) {
                            backend
                                .update_institution(institution.clone(), name, description)
                                .await
                        } else {
                            Ok(())
                        }
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
        })
        .run_on_init(false)
        .on_update({
            clone_states!(loading_state);
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(_) => {
                    loading_state.set(false);
                }
            }
        }),
    );

    let delete_institution = use_command(
        UseCommand::new({
            clone_states!(institution_state, institution_options_state);
            |backend| async move {
                if let Some(index) = &*institution_state {
                    if let Some(institution) = institution_options_state.get(*index) {
                        backend.delete_institution(institution.clone()).await
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
        })
        .run_on_init(false)
        .on_update({
            clone_states!(institution_state, loading_state, get_institutions);
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(_) => {
                    loading_state.set(false);
                    institution_state.set(None);
                    get_institutions.run();
                }
            }
        }),
    );

    let create_click = move |_| create_institution.run();
    let edit_click = move |_| edit_institution.run();
    let delete_click = move |_| delete_institution.run();

    let update_fields = {
        clone_states!(
            institution_options_state,
            institution_name_state,
            institution_name_error_state,
            institution_description_state,
            institution_description_error_state
        );
        move |maybe_index| match maybe_index {
            Some(index) => {
                if let Some::<&Institution>(institution) = institution_options_state.get(index) {
                    institution_name_state.set(institution.name.clone());
                    institution_name_error_state.set(None);
                    institution_description_state
                        .set(institution.description.clone().unwrap_or_default());
                    institution_description_error_state.set(None);
                }
            }
            None => {
                institution_name_state.set(String::new());
                institution_name_error_state.set(None);
                institution_description_state.set(String::new());
                institution_description_error_state.set(None);
            }
        }
    };

    let institution_form = match *institution_state {
        None => html! {
            <div class="edit-institutions-create">
                <Input
                    state={institution_name_state}
                    label="Name"
                    on_submit={create_click.clone()}
                    required={true}
                    error={(*institution_name_error_state).clone()}
                />
                <TextArea
                    state={institution_description_state}
                    label="Description"
                    error={(*institution_description_error_state).clone()}
                />
                <div class="edit-institutions-create-actions">
                    <Button
                        text="Create"
                        on_click={create_click}
                    />
                </div>
            </div>
        },
        Some(_) => html! {
            <div class="edit-institutions-edit">
                <Input
                    state={institution_name_state}
                    label="Name"
                    on_submit={edit_click.clone()}
                    required={true}
                    error={(*institution_name_error_state).clone()}
                />
                <TextArea
                    state={institution_description_state}
                    label="Description"
                    error={(*institution_description_error_state).clone()}
                />
                <div class="edit-institutions-edit-actions">
                    <Button
                        text="Save"
                        on_click={edit_click}
                    />
                    <Button
                        text="Delete"
                        on_click={delete_click}
                        style={ButtonStyle::Danger}
                    />
                </div>
            </div>
        },
    };

    let select_null_option = {
        clone_states!(institution_state);
        move |_| {
            institution_state.set(None);
        }
    };

    let leave_click = move |_| {
        subview.pop();
        on_exit.emit(());
    };

    html! {
        <div class="subview edit-institutions">
            <div class="edit-institutions-title">
                <h2>{"Institutions"}</h2>
                <IconButton
                    name="xmark-solid"
                    size={IconButtonSize::Large}
                    on_click={leave_click}
                />
            </div>
            <div class="edit-institutions-form">
                <SelectNullable
                    state={institution_state}
                    on_change={update_fields}
                    label="Institution"
                    null_label="Create new..."
                    options={institution_options}
                    action_icon="plus-solid"
                    on_action={select_null_option}
                />
                {institution_form}
            </div>
            <LoadingOverlay state={loading_state} />
        </div>
    }
}
