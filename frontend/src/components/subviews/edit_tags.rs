use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::util::*;
use crate::validation::*;
use commands::FrontendCommands;
use common::*;
use yew::prelude::*;

/// Tag configuration subview properties.
#[derive(Clone, PartialEq, Properties)]
pub struct EditTagsProps {
    /// The callback called when the subview is exited.
    #[prop_or_default]
    pub on_exit: Callback<()>,
}

/// The tag editing subview.
#[function_component]
pub fn EditTags(props: &EditTagsProps) -> Html {
    let EditTagsProps { on_exit } = props.clone();

    let tag_state = use_state(|| None);
    let tag_options_state = use_state(Vec::new);
    let tag_name_state = use_state(String::new);
    let tag_name_error_state = use_state(|| None::<String>);
    let tag_description_state = use_state(String::new);
    let tag_description_error_state = use_state(|| None::<String>);
    let new_tag_state = use_state(|| None::<Tag>);
    let loading_state = use_state(|| false);

    let subview = use_subview();

    let tag_options = tag_options_state
        .iter()
        .map(|option: &Tag| option.name.clone())
        .collect::<Vec<_>>();

    let get_tags = use_command(UseCommand::new({
        clone_states!(tag_state, tag_options_state, new_tag_state);
        |backend| async move {
            let tags = backend.tags().await?;

            if let Some(new_tag) = &*new_tag_state {
                if let Some(index) = tags.iter().position(|tag| tag.id == new_tag.id) {
                    tag_state.set(Some(index));
                }

                new_tag_state.set(None);
            }

            tag_options_state.set(tags);
            Ok(())
        }
    }));

    let create_tag = use_command(
        UseCommand::new({
            clone_states!(
                tag_name_state,
                tag_name_error_state,
                tag_description_state,
                tag_description_error_state,
            );
            |backend| async move {
                if let Some((name, description)) = validate_all!(
                    validate(tag_name_state, tag_name_error_state, validate_tag_name),
                    validate(
                        tag_description_state,
                        tag_description_error_state,
                        validate_tag_description
                    )
                ) {
                    backend.create_tag(name, description).await.map(Some)
                } else {
                    Ok(None)
                }
            }
        })
        .run_on_init(false)
        .on_update({
            clone_states!(
                tag_name_state,
                tag_description_state,
                new_tag_state,
                loading_state,
                get_tags
            );
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);

                    // TODO: handle future expected errors, e.g. duplicate tag name
                    if let Ok(Some(tag)) = res {
                        new_tag_state.set(Some(tag.clone()));
                        tag_name_state.set(String::new());
                        tag_description_state.set(String::new());
                    }

                    get_tags.run();
                }
            }
        }),
    );

    let edit_tag = use_command(
        UseCommand::new({
            clone_states!(
                tag_state,
                tag_options_state,
                tag_name_state,
                tag_name_error_state,
                tag_description_state,
                tag_description_error_state,
            );
            |backend| async move {
                if let Some((name, description)) = validate_all!(
                    validate(tag_name_state, tag_name_error_state, validate_tag_name),
                    validate(
                        tag_description_state,
                        tag_description_error_state,
                        validate_tag_description
                    )
                ) {
                    if let Some(index) = &*tag_state {
                        if let Some(tag) = tag_options_state.get(*index) {
                            backend.update_tag(tag.clone(), name, description).await
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

    let delete_tag = use_command(
        UseCommand::new({
            clone_states!(tag_state, tag_options_state);
            |backend| async move {
                if let Some(index) = &*tag_state {
                    if let Some(tag) = tag_options_state.get(*index) {
                        backend.delete_tag(tag.clone()).await
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
            clone_states!(tag_state, loading_state, get_tags);
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(_) => {
                    loading_state.set(false);
                    tag_state.set(None);
                    get_tags.run();
                }
            }
        }),
    );

    let create_click = move |_| create_tag.run();
    let edit_click = move |_| edit_tag.run();
    let delete_click = move |_| delete_tag.run();

    let update_fields = {
        clone_states!(
            tag_options_state,
            tag_name_state,
            tag_name_error_state,
            tag_description_state,
            tag_description_error_state
        );
        move |maybe_index| match maybe_index {
            Some(index) => {
                if let Some::<&Tag>(tag) = tag_options_state.get(index) {
                    tag_name_state.set(tag.name.clone());
                    tag_name_error_state.set(None);
                    tag_description_state.set(tag.description.clone().unwrap_or_default());
                    tag_description_error_state.set(None);
                }
            }
            None => {
                tag_name_state.set(String::new());
                tag_name_error_state.set(None);
                tag_description_state.set(String::new());
                tag_description_error_state.set(None);
            }
        }
    };

    let tag_form = match *tag_state {
        None => html! {
            <div class="edit-tags-create">
                <Input
                    state={tag_name_state}
                    label="Name"
                    on_submit={create_click.clone()}
                    required={true}
                    error={(*tag_name_error_state).clone()}
                />
                <TextArea
                    state={tag_description_state}
                    label="Description"
                    error={(*tag_description_error_state).clone()}
                />
                <div class="edit-tags-create-actions">
                    <Button
                        text="Create"
                        on_click={create_click}
                    />
                </div>
            </div>
        },
        Some(_) => html! {
            <div class="edit-tags-edit">
                <Input
                    state={tag_name_state}
                    label="Name"
                    on_submit={edit_click.clone()}
                    required={true}
                    error={(*tag_name_error_state).clone()}
                />
                <TextArea
                    state={tag_description_state}
                    label="Description"
                    error={(*tag_description_error_state).clone()}
                />
                <div class="edit-tags-edit-actions">
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
        clone_states!(tag_state);
        move |_| {
            tag_state.set(None);
        }
    };

    let leave_click = move |_| {
        subview.pop();
        on_exit.emit(());
    };

    html! {
        <div class="subview edit-tags">
            <div class="edit-tags-title">
                <h2>{"Tags"}</h2>
                <IconButton
                    name="xmark-solid"
                    size={IconButtonSize::Large}
                    on_click={leave_click}
                />
            </div>
            <div class="edit-tags-form">
                <SelectNullable
                    state={tag_state}
                    on_change={update_fields}
                    label="Tag"
                    null_label="Create new..."
                    options={tag_options}
                    action_icon="plus-solid"
                    on_action={select_null_option}
                />
                {tag_form}
            </div>
            <LoadingOverlay state={loading_state} />
        </div>
    }
}
