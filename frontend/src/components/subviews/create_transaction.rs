use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::util::*;
use crate::validation::*;
use commands::FrontendCommands;
use common::*;
use yew::prelude::*;

/// Transaction creation properties.
#[derive(Clone, PartialEq, Properties)]
pub struct CreateTransactionProps {
    /// The account to create the transaction under.
    pub account: Account,
    /// The callback called when the subview is exited. The parameter passed
    /// to the function is the new transaction, or `None` if the transaction
    /// was not created.
    #[prop_or_default]
    pub on_exit: Callback<Option<AccountTransaction>>,
}

/// Transaction creation subview.
#[function_component]
pub fn CreateTransaction(props: &CreateTransactionProps) -> Html {
    let CreateTransactionProps { account, on_exit } = props.clone();

    let institutions_state = use_state(Vec::new);
    let categories_state = use_state(Vec::new);
    let subcategories_state = use_state(Vec::new);
    let tags_state = use_state(Vec::new);

    let transaction_name_state = use_state(String::new);
    let transaction_name_error_state = use_state(|| None::<String>);
    let transaction_description_state = use_state(String::new);
    let transaction_description_error_state = use_state(|| None::<String>);
    let transaction_amount_state = use_state(|| NumberState::new(0.0).decimals(2));
    let transaction_type_state = use_state(|| None);
    let transaction_type_error_state = use_state(|| None::<String>);
    let transaction_institution_state = use_state(|| None::<usize>);
    let transaction_institution_error_state = use_state(|| None::<String>);
    let transaction_date_state = use_state(DatePickerState::new_today);
    let transaction_date_error_state = use_state(|| None::<String>);
    let transaction_category_state = use_state(|| None::<usize>);
    let transaction_category_error_state = use_state(|| None::<String>);
    let transaction_subcategory_state = use_state(|| None::<usize>);
    let transaction_subcategory_error_state = use_state(|| None::<String>);
    let transaction_tags_state = use_state(Vec::new);

    let loading_state = use_state(|| false);

    let subview = use_subview();

    let _get_institutions = use_command(UseCommand::new({
        clone_states!(institutions_state);
        |backend| async move {
            let institutions = backend.institutions().await?;
            institutions_state.set(institutions);
            Ok(())
        }
    }));

    let _get_categories = use_command(UseCommand::new({
        clone_states!(categories_state);
        |backend| async move {
            let categories = backend.categories().await?;
            categories_state.set(categories);
            Ok(())
        }
    }));

    let get_subcategories = use_command(
        UseCommand::new({
            clone_states!(
                transaction_category_state,
                categories_state,
                subcategories_state
            );
            |backend| async move {
                match *transaction_category_state {
                    Some(category_index) => match categories_state.get(category_index) {
                        Some(category) => {
                            let subcategories =
                                backend.subcategories_within(category.clone()).await?;
                            subcategories_state.set(subcategories);
                        }
                        None => {
                            categories_state.set(Vec::new());
                        }
                    },
                    None => {
                        subcategories_state.set(Vec::new());
                    }
                }

                Ok(())
            }
        })
        .run_on_init(false),
    );

    let _get_tags = use_command(UseCommand::new({
        clone_states!(tags_state);
        |backend| async move {
            let tags = backend.tags().await?;
            tags_state.set(tags);
            Ok(())
        }
    }));

    let create_transaction = use_command(
        UseCommand::new({
            clone_states!(
                transaction_name_state,
                transaction_name_error_state,
                transaction_description_state,
                transaction_description_error_state,
                transaction_amount_state,
                transaction_type_state,
                transaction_type_error_state,
                transaction_institution_state,
                transaction_institution_error_state,
                transaction_date_state,
                transaction_date_error_state,
                transaction_category_state,
                transaction_category_error_state,
                transaction_subcategory_state,
                transaction_subcategory_error_state,
                transaction_tags_state,
                institutions_state,
                categories_state,
                subcategories_state,
                tags_state,
            );
            |backend| async move {
                let transaction_institution = transaction_institution_state
                    .and_then(|index| institutions_state.get(index).cloned());
                let transaction_category = transaction_category_state
                    .and_then(|index| categories_state.get(index).cloned());
                let transaction_category2 = transaction_category.clone();
                let transaction_subcategory = transaction_subcategory_state
                    .and_then(|index| subcategories_state.get(index).cloned());
                let tags = transaction_tags_state
                    .iter()
                    .map(|index| {
                        let tag: &Tag = &tags_state[*index];
                        tag.clone()
                    })
                    .collect::<Vec<_>>();

                if let Some((
                    name,
                    description,
                    transaction_type,
                    institution,
                    date,
                    category,
                    subcategory,
                )) = validate_all!(
                    validate(
                        transaction_name_state,
                        transaction_name_error_state,
                        validate_transaction_name
                    ),
                    validate(
                        transaction_description_state,
                        transaction_description_error_state,
                        validate_transaction_description
                    ),
                    validate(
                        transaction_type_state,
                        transaction_type_error_state,
                        validate_transaction_type
                    ),
                    validate_static(
                        transaction_institution,
                        transaction_institution_error_state,
                        validate_transaction_institution
                    ),
                    validate(
                        transaction_date_state,
                        transaction_date_error_state,
                        validate_transaction_date
                    ),
                    validate_static(
                        transaction_category,
                        transaction_category_error_state,
                        validate_transaction_category
                    ),
                    validate_static_with(
                        transaction_subcategory,
                        transaction_subcategory_error_state,
                        validate_transaction_subcategory,
                        &transaction_category2
                    )
                ) {
                    let amount = **transaction_amount_state;
                    backend
                        .create_transaction(
                            account,
                            name,
                            description,
                            amount,
                            transaction_type,
                            institution,
                            date,
                            category,
                            subcategory,
                            tags,
                        )
                        .await
                        .map(Some)
                } else {
                    Ok(None)
                }
            }
        })
        .run_on_init(false)
        .on_update({
            clone_states!(loading_state, subview, on_exit);
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);

                    // TODO: handle future expected errors
                    if let Ok(Some(transaction)) = res {
                        subview.pop();
                        on_exit.emit(Some(transaction.clone()));
                    }
                }
            }
        }),
    );

    let cancel_click = move |_| {
        subview.pop();
        on_exit.emit(None);
    };

    let create_click = move |_| create_transaction.run();

    let fetch_subcategories = move |_| {
        get_subcategories.run();
    };

    let institution_names = institutions_state
        .iter()
        .map(|institution| institution.name.clone())
        .collect::<Vec<_>>();
    let category_names = categories_state
        .iter()
        .map(|category| category.name.clone())
        .collect::<Vec<_>>();
    let subcategory_names = subcategories_state
        .iter()
        .map(|subcategory| subcategory.name.clone())
        .collect::<Vec<_>>();
    let tag_names = tags_state
        .iter()
        .map(|tag| tag.name.clone())
        .collect::<Vec<_>>();

    html! {
        <div class="subview create-transaction">
            <div class="create-transaction-title">
                <h2>{"Create transaction"}</h2>
            </div>
            <div class="create-transaction-form">
                // TODO: add form control actions
                <Input
                    state={transaction_name_state}
                    label="Name"
                    required={true}
                    on_submit={create_click.clone()}
                    error={(*transaction_name_error_state).clone()}
                />
                <TextArea
                    state={transaction_description_state}
                    label="Description"
                    required={false}
                    error={(*transaction_description_error_state).clone()}
                />
                <NumberInput<f64>
                    state={transaction_amount_state}
                    label="Amount"
                    required={true}
                />
                <SelectNullableEnum<TransactionType>
                    state={transaction_type_state}
                    label="Type"
                    required={true}
                    error={(*transaction_type_error_state).clone()}
                />
                <SelectNullable
                    state={transaction_institution_state}
                    options={institution_names}
                    label="Institution"
                    required={true}
                    error={(*transaction_institution_error_state).clone()}
                />
                <DatePicker
                    state={transaction_date_state}
                    label="Date"
                    required={true}
                    error={(*transaction_date_error_state).clone()}
                />
                <SelectNullable
                    state={transaction_category_state}
                    on_change={fetch_subcategories}
                    options={category_names}
                    label="Category"
                    required={true}
                    error={(*transaction_category_error_state).clone()}
                />
                <SelectNullable
                    state={transaction_subcategory_state}
                    options={subcategory_names}
                    label="Subcategory"
                    required={false}
                    error={(*transaction_subcategory_error_state).clone()}
                />
                <Chips
                    state={transaction_tags_state}
                    options={tag_names}
                    label="Tags"
                />
            </div>
            <div class="create-transaction-actions">
                <Button
                    text="Create"
                    on_click={create_click}
                />
                <Button
                    text="Cancel"
                    style={ButtonStyle::Secondary}
                    on_click={cancel_click}
                />
            </div>
            <LoadingOverlay state={loading_state} />
        </div>
    }
}
