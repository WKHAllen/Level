use crate::components::base::*;
use crate::components::misc::*;
use crate::components::subviews::*;
use crate::hooks::*;
use crate::util::*;
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

            let new_transaction = {
                clone_states!(
                    subview,
                    selected_account_index_state,
                    loaded_transactions_state
                );
                move |_| {
                    if let Some(index) = &*selected_account_index_state {
                        if let Some(accounts) = &*accounts_state {
                            if let Some(account) = accounts.get(*index) {
                                clone_states!(account);
                                let on_exit = {
                                    clone_states!(loaded_transactions_state);
                                    move |maybe_transaction: Option<AccountTransaction>| {
                                        if let Some(transaction) = maybe_transaction {
                                            // TODO: put the new transaction into the correct place in the loaded transactions vector
                                            let mut loaded_transactions =
                                                (*loaded_transactions_state).clone();
                                            loaded_transactions.push(transaction);
                                            loaded_transactions_state.set(loaded_transactions);
                                        }
                                    }
                                };
                                subview.push(html! {
                                    <CreateTransaction {account} {on_exit} />
                                });
                            }
                        }
                    }
                }
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
                .map(|transaction| {
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
                                        <div class="account-transactions-actions-new">
                                            <Tooltip text="New transaction">
                                                <IconButton
                                                    name="plus-solid"
                                                    size={IconButtonSize::Small}
                                                    on_click={new_transaction}
                                                />
                                            </Tooltip>
                                        </div>
                                        // TODO: more transaction actions
                                    </div>
                                </div>
                                <div class="account-transactions-loading">
                                    {account_transactions_loading}
                                </div>
                                <div class="account-transactions-list">
                                    {account_transactions}
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
