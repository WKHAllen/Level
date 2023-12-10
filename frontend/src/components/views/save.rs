use crate::components::base::*;
use crate::components::misc::*;
use crate::components::subviews::*;
use crate::hooks::*;
use crate::util::*;
use crate::validation::*;
use crate::view::View;
use commands::FrontendCommands;
use common::*;
use yew::prelude::*;

/// The number of transactions to request in one batch.
const TRANSACTION_BATCH_LIMIT: usize = 100;

/// The open save page view.
#[function_component]
pub fn Save() -> Html {
    let save_info_state = use_state(|| None);
    let accounts_state = use_state(|| None);
    let selected_account_index_state = use_state(|| None);
    let loaded_transactions_state = use_state(Vec::new);
    let all_transactions_loaded_state = use_state(|| false);
    let info_pane_state = use_state(|| true);
    let stats_pane_state = use_state(|| false);
    let answers_pane_state = use_state(|| false);

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

    let view = use_view();
    let subview = use_subview();
    let alert = use_alert();

    let _get_save_info = use_command(UseCommand::new({
        clone_states!(save_info_state);
        |backend| async move {
            let save_info = backend.save_info().await?;
            save_info_state.set(Some(save_info));
            Ok(())
        }
    }));

    let get_accounts = use_command(UseCommand::new({
        clone_states!(accounts_state, selected_account_index_state);
        |backend| async move {
            let accounts = backend.accounts().await?;
            accounts_state.set(Some(accounts));
            selected_account_index_state.set(Some(0));
            Ok(())
        }
    }));

    let get_transactions = use_command(
        UseCommand::new({
            clone_states!(
                accounts_state,
                selected_account_index_state,
                loaded_transactions_state,
                all_transactions_loaded_state
            );
            move |backend| async move {
                match &*selected_account_index_state {
                    Some(index) => match &*accounts_state {
                        Some(accounts) => match accounts.get(*index) {
                            Some::<&Account>(account) => {
                                let mut transactions = backend
                                    .transaction_batch(
                                        account.clone(),
                                        loaded_transactions_state.len(),
                                        TRANSACTION_BATCH_LIMIT,
                                    )
                                    .await?;

                                if transactions.len() < TRANSACTION_BATCH_LIMIT {
                                    all_transactions_loaded_state.set(true);
                                }

                                let loaded_transactions = (*loaded_transactions_state).clone();
                                transactions.extend(loaded_transactions);
                                loaded_transactions_state.set(transactions);

                                Ok(())
                            }
                            None => Ok(()),
                        },
                        None => Ok(()),
                    },
                    None => Ok(()),
                }
            }
        })
        .run_on_init(false),
    );

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
                accounts_state,
                selected_account_index_state,
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

                    if let Some(index) = &*selected_account_index_state {
                        if let Some(accounts) = &*accounts_state {
                            if let Some(account) = accounts.get(*index) {
                                backend
                                    .create_transaction(
                                        account.clone(),
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
            clone_states!(loading_state, loaded_transactions_state);
            move |value| match value {
                UseCommandState::Init => {}
                UseCommandState::Loading => {
                    loading_state.set(true);
                }
                UseCommandState::Resolved(res) => {
                    loading_state.set(false);

                    // TODO: handle future expected errors
                    if let Ok(Some(transaction)) = res {
                        // TODO: put the new transaction into the correct place in the loaded transactions vector
                        let mut loaded_transactions = (*loaded_transactions_state).clone();
                        loaded_transactions.push(transaction.clone());
                        loaded_transactions_state.set(loaded_transactions);
                    }
                }
            }
        }),
    );

    let run_create_transaction = move |_| create_transaction.run();

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

    use_effect_with(selected_account_index_state.clone(), {
        clone_states!(
            loaded_transactions_state,
            all_transactions_loaded_state,
            get_transactions
        );
        move |_| {
            loaded_transactions_state.set(Vec::new());
            all_transactions_loaded_state.set(false);
            get_transactions.run();
        }
    });

    match &*save_info_state {
        None => html! { <Loading /> },
        Some(save_info) => {
            let new_account = {
                clone_states!(subview, get_accounts);
                move |_| {
                    let on_exit = {
                        clone_states!(get_accounts);
                        move |maybe_account: Option<Account>| {
                            if maybe_account.is_some() {
                                get_accounts.run();
                            }
                        }
                    };
                    subview.push(html! {
                        <CreateAccount {on_exit} />
                    });
                }
            };

            let account_selection = match &*accounts_state {
                None => html! { <Loading /> },
                Some(accounts) => accounts
                    .iter()
                    .enumerate()
                    .map(|(index, account)| {
                        let onclick = {
                            clone_states!(selected_account_index_state);
                            move |_| {
                                selected_account_index_state.set(Some(index));
                            }
                        };

                        html! {
                            <div class="account-select-button" {onclick}>
                                <span>{&account.name}</span>
                            </div>
                        }
                    })
                    .collect::<Html>(),
            };

            let account_transactions_loading = if let Some(res) = get_transactions.value() {
                if let Err(err) = res {
                    view.set(View::Home);
                    alert.open(
                        UseAlert::new()
                            .title("Application Error")
                            .text(&err.to_string()),
                    );
                }

                html! {}
            } else {
                html! { <Loading /> }
            };

            let account_transactions = loaded_transactions_state
                .iter()
                .map(|_transaction| {
                    html! {
                        <div class="account-transaction">
                            // TODO: display transaction
                            <span>{"Transaction"}</span>
                        </div>
                    }
                })
                .collect::<Html>();

            html! {
                <div class="view save">
                    <div class="save-header bg-4">
                        <div class="save-title">
                            <span>{&save_info.name}</span>
                        </div>
                        <div class="save-actions">
                            // TODO: save actions
                        </div>
                    </div>
                    <div class="save-body">
                        <div class="save-left bg-3">
                            <div class="save-accounts">
                                <div class="save-accounts-banner">
                                    <div class="save-accounts-title">
                                        <span>{"Accounts"}</span>
                                    </div>
                                    <div class="save-accounts-actions">
                                        <div class="save-accounts-actions-new">
                                            <Tooltip text="New account">
                                                <IconButton
                                                    name="plus-solid"
                                                    size={IconButtonSize::Small}
                                                    on_click={new_account}
                                                />
                                            </Tooltip>
                                        </div>
                                        // TODO: more account actions
                                    </div>
                                </div>
                                <div class="save-accounts-list">
                                    {account_selection}
                                </div>
                            </div>
                        </div>
                        <div class="save-main bg-2">
                            <div class="account-transactions">
                                <div class="account-transactions-header">
                                    <div class="account-transactions-title">
                                        <span>{"Transactions"}</span>
                                    </div>
                                    <div class="account-transactions-actions">
                                        // TODO: transaction actions
                                    </div>
                                </div>
                                <div class="account-transactions-loading">
                                    {account_transactions_loading}
                                </div>
                                <div class="account-transactions-list">
                                    {account_transactions}
                                </div>
                                <div class="account-transactions-new">
                                    <div class="account-transactions-new-input">
                                        <Input
                                            state={transaction_name_state}
                                            label="Name"
                                            required={true}
                                            compact={true}
                                            on_submit={run_create_transaction.clone()}
                                            error={(*transaction_name_error_state).clone()}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <TextArea
                                            state={transaction_description_state}
                                            label="Description"
                                            required={false}
                                            compact={true}
                                            error={(*transaction_description_error_state).clone()}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <NumberInput<f64>
                                            state={transaction_amount_state}
                                            label="Amount"
                                            required={true}
                                            compact={true}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <SelectNullableEnum<TransactionType>
                                            state={transaction_type_state}
                                            label="Type"
                                            required={true}
                                            compact={true}
                                            position={SelectPopupPosition::Above}
                                            error={(*transaction_type_error_state).clone()}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <SelectNullable
                                            state={transaction_institution_state}
                                            options={institution_names}
                                            label="Institution"
                                            required={true}
                                            compact={true}
                                            position={SelectPopupPosition::Above}
                                            error={(*transaction_institution_error_state).clone()}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <DatePicker
                                            state={transaction_date_state}
                                            label="Date"
                                            required={true}
                                            compact={true}
                                            position={DatePickerPopupPosition::Above}
                                            error={(*transaction_date_error_state).clone()}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <SelectNullable
                                            state={transaction_category_state}
                                            on_change={fetch_subcategories}
                                            options={category_names}
                                            label="Category"
                                            required={true}
                                            compact={true}
                                            position={SelectPopupPosition::Above}
                                            error={(*transaction_category_error_state).clone()}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <SelectNullable
                                            state={transaction_subcategory_state}
                                            options={subcategory_names}
                                            label="Subcategory"
                                            required={false}
                                            compact={true}
                                            position={SelectPopupPosition::Above}
                                            error={(*transaction_subcategory_error_state).clone()}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <Chips
                                            state={transaction_tags_state}
                                            options={tag_names}
                                            label="Tags"
                                            compact={true}
                                            position={ChipsPopupPosition::Above}
                                        />
                                    </div>
                                    <div class="account-transactions-new-input">
                                        <button
                                            class="account-transaction-create-button"
                                            onclick={move |_| run_create_transaction(())}
                                        >
                                            <Icon
                                                name="plus-solid"
                                                size={IconSize::Small}
                                            />
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="save-right bg-3">
                            <div class="account-info">
                                <ExpandablePane state={info_pane_state} label="Info">
                                    // TODO: info pane
                                    <span>{"Info pane"}</span>
                                </ExpandablePane>
                                <ExpandablePane state={stats_pane_state} label="Stats">
                                    // TODO: stats pane
                                    <div>{"Stats pane"}</div>
                                </ExpandablePane>
                                <ExpandablePane state={answers_pane_state} label="Answers">
                                    // TODO: answers pane
                                    <span>{"Answers pane"}</span>
                                </ExpandablePane>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
