use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
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
    let transaction_description_state = use_state(String::new);
    let transaction_amount_state = use_state(|| NumberState::new(0.0).decimals(2));
    let transaction_type_state = use_state(|| None);
    let transaction_type_error_state = use_state(|| None);
    let transaction_institution_state = use_state(|| None);
    let transaction_institution_error_state = use_state(|| None);
    let transaction_date_state = use_state(DatePickerState::new_today);
    let transaction_category_state = use_state(|| None);
    let transaction_category_error_state = use_state(|| None);
    let transaction_subcategory_state = use_state(|| None);
    let transaction_tags_state = use_state(Vec::new);

    let loading_state = use_state(|| false);

    let subview = use_subview();

    let _get_institutions = use_command(UseCommand::new({
        let institutions_state = institutions_state.clone();
        |backend| async move {
            let institutions = backend.institutions().await?;
            institutions_state.set(institutions);
            Ok(())
        }
    }));

    let _get_categories = use_command(UseCommand::new({
        let categories_state = categories_state.clone();
        |backend| async move {
            let categories = backend.categories().await?;
            categories_state.set(categories);
            Ok(())
        }
    }));

    let get_subcategories = use_command(
        UseCommand::new({
            let transaction_category_state = transaction_category_state.clone();
            let categories_state = categories_state.clone();
            let subcategories_state = subcategories_state.clone();
            |backend| async move {
                match *transaction_category_state {
                    Some(category_index) => match categories_state.get(category_index) {
                        Some::<&Category>(category) => {
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
        let tags_state = tags_state.clone();
        |backend| async move {
            let tags = backend.tags().await?;
            tags_state.set(tags);
            Ok(())
        }
    }));

    let create_transaction = use_command(
        UseCommand::new({
            let transaction_name_state = transaction_name_state.clone();
            let transaction_description_state = transaction_description_state.clone();
            let transaction_amount_state = transaction_amount_state.clone();
            let transaction_type_state = transaction_type_state.clone();
            let transaction_institution_state = transaction_institution_state.clone();
            let transaction_date_state = transaction_date_state.clone();
            let transaction_category_state = transaction_category_state.clone();
            let transaction_subcategory_state = transaction_subcategory_state.clone();
            let transaction_tags_state = transaction_tags_state.clone();
            |backend| async move {
                // let transaction_name = (*transaction_name_state).clone();
                // let transaction_description = (*transaction_description_state).clone();
                // let transaction_amount = **transaction_amount_state;
                // let transaction_type = (*transaction_type_state).unwrap(); // `None` case handled by validation
                // let transaction_institution = (*transaction_institution_state).unwrap(); // `None` case handled by validation
                // let transaction_date = (**transaction_date_state).unwrap(); // `None` case handled by validation
                // let transaction_category = (*transaction_category_state).unwrap(); // `None` case handled by validation
                // let transaction_subcategory = *transaction_subcategory_state;
                // let transaction_tags = *transaction_tags_state;
                // backend
                //     .create_transaction(
                //         account,
                //         transaction_name,
                //         transaction_description,
                //         transaction_amount,
                //         transaction_type,
                //         transaction_institution,
                //         transaction_date,
                //         transaction_category,
                //         transaction_subcategory,
                //         transaction_tags,
                //     )
                //     .await
                Ok(AccountTransaction {
                    id: "".to_owned(),
                    account_id: "".to_owned(),
                    name: "".to_owned(),
                    description: None,
                    amount: 0.0,
                    transaction_type: "".to_owned(),
                    institution_id: "".to_owned(),
                    transaction_date: chrono::NaiveDateTime::MIN,
                    category_id: "".to_owned(),
                    subcategory_id: None,
                    reconciled: false,
                    created_at: chrono::NaiveDateTime::MIN,
                    edited_at: None,
                    reconciled_at: None,
                })
            }
        })
        .run_on_init(false)
        .on_update({
            let loading_state = loading_state.clone();
            let subview = subview.clone();
            let on_exit = on_exit.clone();
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);

                    if let Ok(transaction) = res {
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

    let create_click = {
        let transaction_type_state = transaction_type_state.clone();
        let transaction_institution_state = transaction_institution_state.clone();
        let transaction_category_state = transaction_category_state.clone();
        move |_| {
            // TODO: validate all fields
            if transaction_type_state.is_none() {
                transaction_type_error_state.set(Some("Please select a transaction type"));
            } else if transaction_institution_state.is_none() {
                transaction_institution_error_state.set(Some("Please select an institution"));
            } else if transaction_category_state.is_none() {
                transaction_category_error_state.set(Some("Please select a category"));
            } else {
                create_transaction.run();
            }
        }
    };

    let fetch_subcategories = move |_| {
        get_subcategories.run();
    };

    let tag_options = tags_state
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
                />
                <TextArea
                    state={transaction_description_state}
                    label="Description"
                    required={false}
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
                />
                <SelectNullable
                    state={transaction_institution_state}
                    label="Institution"
                    required={true}
                >
                    // TODO: render institution options
                </SelectNullable>
                <DatePicker
                    state={transaction_date_state}
                    label="Date"
                    required={true}
                />
                <SelectNullable
                    state={transaction_category_state}
                    on_change={fetch_subcategories}
                    label="Category"
                    required={true}
                >
                    // TODO: render category options
                </SelectNullable>
                <SelectNullable
                    state={transaction_subcategory_state}
                    label="Subcategory"
                    required={false}
                >
                    // TODO: render subcategory options
                </SelectNullable>
                <Chips
                    state={transaction_tags_state}
                    options={tag_options}
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
