use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::view::View;
use commands::FrontendCommands;
use common::Account;
use yew::prelude::*;

/// The number of transactions to request in one batch.
const TRANSACTION_BATCH_LIMIT: usize = 100;

/// The open save page view.
#[function_component]
pub fn Save() -> Html {
    let selected_account_index_state = use_state(|| None);
    let loaded_transactions_state = use_state(Vec::new);
    let all_transactions_loaded_state = use_state(|| false);
    let info_pane_state = use_state(|| true);
    let stats_pane_state = use_state(|| false);
    let answers_pane_state = use_state(|| false);

    let view = use_view();
    let alert = use_alert();

    let save_data = use_command(UseCommand::new(|backend| async move {
        let save_info = backend.save_info().await?;
        let accounts = backend.accounts().await?;
        Ok((save_info, accounts))
    }));

    let get_transactions = use_command(
        UseCommand::new({
            let selected_account_index_state = selected_account_index_state.clone();
            let loaded_transactions_state = loaded_transactions_state.clone();
            let all_transactions_loaded_state = all_transactions_loaded_state.clone();
            let save_data = save_data.clone();
            move |backend| async move {
                match &*selected_account_index_state {
                    Some(index) => match &*save_data {
                        UseCommandState::Resolved(res) => match res {
                            Ok((_, accounts)) => match accounts.get(*index) {
                                Some::<&Account>(account) => {
                                    let transactions = backend
                                        .transaction_batch(
                                            account.clone(),
                                            loaded_transactions_state.len(),
                                            TRANSACTION_BATCH_LIMIT,
                                        )
                                        .await?;

                                    if transactions.len() < TRANSACTION_BATCH_LIMIT {
                                        all_transactions_loaded_state.set(true);
                                    }

                                    let mut loaded_transactions =
                                        (*loaded_transactions_state).clone();
                                    loaded_transactions.extend(transactions);
                                    loaded_transactions_state.set(loaded_transactions);

                                    Ok(())
                                }
                                None => Ok(()),
                            },
                            Err(_) => Ok(()),
                        },
                        _ => Ok(()),
                    },
                    None => Ok(()),
                }
            }
        })
        .run_on_init(false),
    );

    use_effect_with(selected_account_index_state.clone(), {
        let loaded_transactions_state = loaded_transactions_state.clone();
        let all_transactions_loaded_state = all_transactions_loaded_state.clone();
        let get_transactions = get_transactions.clone();
        move |_| {
            loaded_transactions_state.set(Vec::new());
            all_transactions_loaded_state.set(false);
            get_transactions.run();
        }
    });

    let new_account_node = use_node_ref();
    use_tooltip(new_account_node.clone(), "New account");

    match &*save_data {
        UseCommandState::Init | UseCommandState::Loading => html! { <Loading /> },
        UseCommandState::Resolved(res) => match res {
            Err(err) => {
                view.set(View::Home);
                alert.open(
                    UseAlert::new()
                        .title("Application Error")
                        .text(&err.to_string()),
                );
                html! {}
            }
            Ok((save_info, accounts)) => {
                let new_account = move |_| {
                    // TODO: push new account view to view stack
                };

                let account_selection = accounts
                    .iter()
                    .enumerate()
                    .map(|(index, account)| {
                        let onclick = {
                            let selected_account_index_state = selected_account_index_state.clone();
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
                    .collect::<Html>();

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
                            <div>
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
                                    // TODO: transaction actions
                                    {account_transactions_loading}
                                    {account_transactions}
                                    // TODO: new transaction
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
        },
    }
}
