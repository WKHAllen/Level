use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::util::*;
use crate::validation::*;
use commands::FrontendCommands;
use common::*;
use yew::prelude::*;

/// Category configuration subview properties.
#[derive(Clone, PartialEq, Properties)]
pub struct EditCategoriesProps {
    /// The callback called when the subview is exited.
    #[prop_or_default]
    pub on_exit: Callback<()>,
}

/// The category editing subview.
#[function_component]
pub fn EditCategories(props: &EditCategoriesProps) -> Html {
    let EditCategoriesProps { on_exit } = props.clone();

    let category_state = use_state(|| None);
    let category_options_state = use_state(Vec::new);
    let category_name_state = use_state(String::new);
    let category_name_error_state = use_state(|| None::<String>);
    let category_description_state = use_state(String::new);
    let category_description_error_state = use_state(|| None::<String>);
    let new_category_state = use_state(|| None::<Category>);
    let loading_state = use_state(|| false);

    let subview = use_subview();

    let category_options = category_options_state
        .iter()
        .map(|option: &Category| option.name.clone())
        .collect::<Vec<_>>();

    let get_categories = use_command(UseCommand::new({
        clone_states!(category_state, category_options_state, new_category_state);
        |backend| async move {
            let categories = backend.categories().await?;

            if let Some(new_category) = &*new_category_state {
                if let Some(index) = categories
                    .iter()
                    .position(|category| category.id == new_category.id)
                {
                    category_state.set(Some(index));
                }

                new_category_state.set(None);
            }

            category_options_state.set(categories);
            Ok(())
        }
    }));

    let create_category = use_command(
        UseCommand::new({
            clone_states!(
                category_name_state,
                category_name_error_state,
                category_description_state,
                category_description_error_state,
            );
            |backend| async move {
                if let Some((name, description)) = validate_all!(
                    validate(
                        category_name_state,
                        category_name_error_state,
                        validate_category_name
                    ),
                    validate(
                        category_description_state,
                        category_description_error_state,
                        validate_category_description
                    )
                ) {
                    backend.create_category(name, description).await.map(Some)
                } else {
                    Ok(None)
                }
            }
        })
        .run_on_init(false)
        .on_update({
            clone_states!(
                category_name_state,
                category_description_state,
                new_category_state,
                loading_state,
                get_categories
            );
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);

                    // TODO: handle future expected errors, e.g. duplicate category name
                    if let Ok(Some(category)) = res {
                        new_category_state.set(Some(category.clone()));
                        category_name_state.set(String::new());
                        category_description_state.set(String::new());
                    }

                    get_categories.run();
                }
            }
        }),
    );

    let edit_category = use_command(
        UseCommand::new({
            clone_states!(
                category_state,
                category_options_state,
                category_name_state,
                category_name_error_state,
                category_description_state,
                category_description_error_state,
            );
            |backend| async move {
                if let Some((name, description)) = validate_all!(
                    validate(
                        category_name_state,
                        category_name_error_state,
                        validate_category_name
                    ),
                    validate(
                        category_description_state,
                        category_description_error_state,
                        validate_category_description
                    )
                ) {
                    if let Some(index) = &*category_state {
                        if let Some(category) = category_options_state.get(*index) {
                            backend
                                .update_category(category.clone(), name, description)
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

    let delete_category = use_command(
        UseCommand::new({
            clone_states!(category_state, category_options_state);
            |backend| async move {
                if let Some(index) = &*category_state {
                    if let Some(category) = category_options_state.get(*index) {
                        backend.delete_category(category.clone()).await
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
            clone_states!(category_state, loading_state, get_categories);
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(_) => {
                    loading_state.set(false);
                    category_state.set(None);
                    get_categories.run();
                }
            }
        }),
    );

    let create_click = move |_| create_category.run();
    let edit_click = move |_| edit_category.run();
    let delete_click = move |_| delete_category.run();

    let update_fields = {
        clone_states!(
            category_options_state,
            category_name_state,
            category_name_error_state,
            category_description_state,
            category_description_error_state
        );
        move |maybe_index| match maybe_index {
            Some(index) => {
                if let Some::<&Category>(category) = category_options_state.get(index) {
                    category_name_state.set(category.name.clone());
                    category_name_error_state.set(None);
                    category_description_state
                        .set(category.description.clone().unwrap_or_default());
                    category_description_error_state.set(None);
                }
            }
            None => {
                category_name_state.set(String::new());
                category_name_error_state.set(None);
                category_description_state.set(String::new());
                category_description_error_state.set(None);
            }
        }
    };

    let category_form = match *category_state {
        None => html! {
            <div class="edit-categories-create">
                <Input
                    state={category_name_state}
                    label="Name"
                    on_submit={create_click.clone()}
                    required={true}
                    error={(*category_name_error_state).clone()}
                />
                <TextArea
                    state={category_description_state}
                    label="Description"
                    error={(*category_description_error_state).clone()}
                />
                <div class="edit-categories-create-actions">
                    <Button
                        text="Create"
                        on_click={create_click}
                    />
                </div>
            </div>
        },
        Some(_) => html! {
            <div class="edit-categories-edit">
                <Input
                    state={category_name_state}
                    label="Name"
                    on_submit={edit_click.clone()}
                    required={true}
                    error={(*category_name_error_state).clone()}
                />
                <TextArea
                    state={category_description_state}
                    label="Description"
                    error={(*category_description_error_state).clone()}
                />
                <div class="edit-categories-edit-actions">
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
        clone_states!(category_state);
        move |_| {
            category_state.set(None);
        }
    };

    let leave_click = move |_| {
        subview.pop();
        on_exit.emit(());
    };

    html! {
        <div class="subview edit-categories">
            <div class="edit-categories-title">
                <h2>{"Categories"}</h2>
                <IconButton
                    name="xmark-solid"
                    size={IconButtonSize::Large}
                    on_click={leave_click}
                />
            </div>
            <div class="edit-categories-form">
                <SelectNullable
                    state={category_state}
                    on_change={update_fields}
                    label="Category"
                    null_label="Create new..."
                    options={category_options}
                    action_icon="plus-solid"
                    on_action={select_null_option}
                />
                {category_form}
            </div>
            <LoadingOverlay state={loading_state} />
        </div>
    }
}
