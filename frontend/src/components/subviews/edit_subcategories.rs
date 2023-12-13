use super::edit_categories::*;
use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::util::*;
use crate::validation::*;
use commands::FrontendCommands;
use common::*;
use yew::prelude::*;

/// Subcategory configuration subview properties.
#[derive(Clone, PartialEq, Properties)]
pub struct EditSubcategoriesProps {
    /// The callback called when the subview is exited. The returned value
    /// is a 2-tuple, with the first element representing whether any changes
    /// were made to the subcategories, and the second element representing
    /// whether any changes were made to the categories.
    #[prop_or_default]
    pub on_exit: Callback<(bool, bool)>,
}

/// The subcategory editing subview.
#[function_component]
pub fn EditSubcategories(props: &EditSubcategoriesProps) -> Html {
    let EditSubcategoriesProps { on_exit } = props.clone();

    let category_state = use_state(|| None);
    let category_options_state = use_state(Vec::new);
    let subcategory_state = use_state(|| None);
    let subcategory_options_state = use_state(Vec::new);
    let subcategory_name_state = use_state(String::new);
    let subcategory_name_error_state = use_state(|| None::<String>);
    let subcategory_description_state = use_state(String::new);
    let subcategory_description_error_state = use_state(|| None::<String>);
    let new_subcategory_state = use_state(|| None::<Subcategory>);
    let loading_state = use_state(|| false);
    let dirty_state = use_state(|| false);
    let category_dirty_state = use_state(|| false);

    let subview = use_subview();

    let category_options = category_options_state
        .iter()
        .map(|option: &Category| option.name.clone())
        .collect::<Vec<_>>();
    let subcategory_options = subcategory_options_state
        .iter()
        .map(|option: &Subcategory| option.name.clone())
        .collect::<Vec<_>>();

    let get_categories = use_command(UseCommand::new({
        clone_states!(category_options_state);
        |backend| async move {
            let categories = backend.categories().await?;
            category_options_state.set(categories);
            Ok(())
        }
    }));

    let get_subcategories = use_command(UseCommand::new({
        clone_states!(
            category_state,
            category_options_state,
            subcategory_state,
            subcategory_options_state,
            new_subcategory_state
        );
        |backend| async move {
            if let Some(index) = &*category_state {
                if let Some::<&Category>(category) = category_options_state.get(*index) {
                    let subcategories = backend.subcategories_within(category.clone()).await?;

                    if let Some(new_subcategory) = &*new_subcategory_state {
                        if let Some(index) = subcategories
                            .iter()
                            .position(|subcategory| subcategory.id == new_subcategory.id)
                        {
                            subcategory_state.set(Some(index));
                        }

                        new_subcategory_state.set(None);
                    }

                    subcategory_options_state.set(subcategories);
                    Ok(())
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        }
    }));

    let create_subcategory = use_command(
        UseCommand::new({
            clone_states!(
                category_state,
                category_options_state,
                subcategory_name_state,
                subcategory_name_error_state,
                subcategory_description_state,
                subcategory_description_error_state
            );
            |backend| async move {
                if let Some((name, description)) = validate_all!(
                    validate(
                        subcategory_name_state,
                        subcategory_name_error_state,
                        validate_subcategory_name
                    ),
                    validate(
                        subcategory_description_state,
                        subcategory_description_error_state,
                        validate_subcategory_description
                    )
                ) {
                    if let Some(index) = &*category_state {
                        if let Some::<&Category>(category) = category_options_state.get(*index) {
                            backend
                                .create_subcategory_within(category.clone(), name, description)
                                .await
                                .map(Some)
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
        })
        .run_on_init(false)
        .on_update({
            clone_states!(
                subcategory_name_state,
                subcategory_description_state,
                new_subcategory_state,
                loading_state,
                dirty_state,
                get_subcategories
            );
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);

                    // TODO: handle future expected errors, e.g. duplicate subcategory name
                    if let Ok(Some(subcategory)) = res {
                        new_subcategory_state.set(Some(subcategory.clone()));
                        subcategory_name_state.set(String::new());
                        subcategory_description_state.set(String::new());
                        dirty_state.set(true);
                    }

                    get_subcategories.run();
                }
            }
        }),
    );

    let edit_subcategory = use_command(
        UseCommand::new({
            clone_states!(
                subcategory_state,
                subcategory_options_state,
                subcategory_name_state,
                subcategory_name_error_state,
                subcategory_description_state,
                subcategory_description_error_state
            );
            |backend| async move {
                if let Some((name, description)) = validate_all!(
                    validate(
                        subcategory_name_state,
                        subcategory_name_error_state,
                        validate_subcategory_name
                    ),
                    validate(
                        subcategory_description_state,
                        subcategory_description_error_state,
                        validate_subcategory_description
                    )
                ) {
                    if let Some(index) = &*subcategory_state {
                        if let Some(subcategory) = subcategory_options_state.get(*index) {
                            backend
                                .update_subcategory(subcategory.clone(), name, description)
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
            clone_states!(loading_state, dirty_state, get_subcategories);
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);

                    // TODO: handle future expected errors, e.g. duplicate subcategory name
                    #[allow(clippy::redundant_pattern_matching)]
                    if let Ok(_) = res {
                        dirty_state.set(true);
                    }

                    get_subcategories.run();
                }
            }
        }),
    );

    let delete_subcategory = use_command(
        UseCommand::new({
            clone_states!(subcategory_state, subcategory_options_state);
            |backend| async move {
                if let Some(index) = &*subcategory_state {
                    if let Some(subcategory) = subcategory_options_state.get(*index) {
                        backend.delete_subcategory(subcategory.clone()).await
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
            clone_states!(
                subcategory_state,
                loading_state,
                dirty_state,
                get_subcategories
            );
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);
                    subcategory_state.set(None);

                    // TODO: handle future expected errors, e.g. duplicate subcategory name
                    #[allow(clippy::redundant_pattern_matching)]
                    if let Ok(_) = res {
                        dirty_state.set(true);
                    }

                    get_subcategories.run();
                }
            }
        }),
    );

    let create_click = move |_| create_subcategory.run();
    let edit_click = move |_| edit_subcategory.run();
    let delete_click = move |_| delete_subcategory.run();

    let update_fields = {
        clone_states!(
            subcategory_options_state,
            subcategory_name_state,
            subcategory_name_error_state,
            subcategory_description_state,
            subcategory_description_error_state
        );
        move |maybe_index| match maybe_index {
            Some(index) => {
                if let Some::<&Subcategory>(subcategory) = subcategory_options_state.get(index) {
                    subcategory_name_state.set(subcategory.name.clone());
                    subcategory_name_error_state.set(None);
                    subcategory_description_state
                        .set(subcategory.description.clone().unwrap_or_default());
                    subcategory_description_error_state.set(None);
                }
            }
            None => {
                subcategory_name_state.set(String::new());
                subcategory_name_error_state.set(None);
                subcategory_description_state.set(String::new());
                subcategory_description_error_state.set(None);
            }
        }
    };

    let subcategory_form = match *subcategory_state {
        None => html! {
            <div class="edit-subcategories-create">
                <Input
                    state={subcategory_name_state}
                    label="Name"
                    on_submit={create_click.clone()}
                    required={true}
                    error={(*subcategory_name_error_state).clone()}
                />
                <TextArea
                    state={subcategory_description_state}
                    label="Description"
                    error={(*subcategory_description_error_state).clone()}
                />
                <div class="edit-subcategories-create-actions">
                    <Button
                        text="Create"
                        on_click={create_click}
                    />
                </div>
            </div>
        },
        Some(_) => html! {
            <div class="edit-subcategories-edit">
                <Input
                    state={subcategory_name_state}
                    label="Name"
                    on_submit={edit_click.clone()}
                    required={true}
                    error={(*subcategory_name_error_state).clone()}
                />
                <TextArea
                    state={subcategory_description_state}
                    label="Description"
                    error={(*subcategory_description_error_state).clone()}
                />
                <div class="edit-subcategories-edit-actions">
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
        clone_states!(subcategory_state);
        move |_| {
            subcategory_state.set(None);
        }
    };

    let configure_categories = {
        clone_states!(
            category_state,
            category_dirty_state,
            subview,
            get_categories
        );
        move |_| {
            let on_exit = {
                clone_states!(category_state, category_dirty_state, get_categories);
                move |dirty| {
                    if dirty {
                        get_categories.run();
                        category_state.set(None);
                        category_dirty_state.set(true);
                    }
                }
            };
            subview.push(html! {
                <EditCategories {on_exit} />
            });
        }
    };

    let update_subcategories = {
        clone_states!(subcategory_state, get_subcategories);
        move |_| {
            get_subcategories.run();
            subcategory_state.set(None);
        }
    };

    let leave_click = {
        clone_states!(subview);
        move |_| {
            subview.pop();
            on_exit.emit((*dirty_state, *category_dirty_state));
        }
    };

    let subcategory_selection = match *category_state {
        Some(_) => html! {
            <>
                <SelectNullable
                    state={subcategory_state}
                    on_change={update_fields}
                    label="Subcategory"
                    null_label="Create new..."
                    options={subcategory_options}
                    action_icon="plus-solid"
                    on_action={select_null_option}
                />
                {subcategory_form}
            </>
        },
        None => html! {
            <div class="edit-subcategories-unselected">
                <span>{"Select a category to configure its subcategories."}</span>
            </div>
        },
    };

    html! {
        <div class="subview edit-subcategories">
            <div class="edit-subcategories-title">
                <h2>{"Subcategories"}</h2>
                <IconButton
                    name="xmark-solid"
                    size={IconButtonSize::Large}
                    on_click={leave_click}
                />
            </div>
            <div class="edit-subcategories-form">
                <SelectNullable
                    state={category_state}
                    on_change={update_subcategories}
                    label="Category"
                    options={category_options}
                    action_icon="ellipsis-solid"
                    on_action={configure_categories}
                />
                {subcategory_selection}
            </div>
            <LoadingOverlay state={loading_state} />
        </div>
    }
}
