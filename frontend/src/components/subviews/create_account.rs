use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use crate::util::*;
use crate::validation::*;
use commands::FrontendCommands;
use common::*;
use yew::prelude::*;

/// Account creation subview properties.
#[derive(Clone, PartialEq, Properties)]
pub struct CreateAccountProps {
    /// The callback called when the subview is exited. The parameter passed
    /// to the function is the new account, or `None` if the account was not
    /// created.
    #[prop_or_default]
    pub on_exit: Callback<Option<Account>>,
}

/// The account creation subview.
#[function_component]
pub fn CreateAccount(props: &CreateAccountProps) -> Html {
    let CreateAccountProps { on_exit } = props.clone();

    let account_type_state = use_state(|| None);
    let account_type_error_state = use_state(|| None);
    let account_name_state = use_state(String::new);
    let account_name_error_state = use_state(|| None);
    let account_description_state = use_state(String::new);
    let account_description_error_state = use_state(|| None);
    let loading_state = use_state(|| false);

    let subview = use_subview();

    let create_account = use_command(
        UseCommand::new({
            clone_states!(
                account_type_state,
                account_type_error_state,
                account_name_state,
                account_name_error_state,
                account_description_state,
                account_description_error_state,
            );
            |backend| async move {
                if let Some((account_type, name, description)) = validate_all!(
                    account_type_state, account_type_error_state, validate_account_type;
                    account_name_state, account_name_error_state, validate_account_name;
                    account_description_state, account_description_error_state, validate_account_description;
                ) {
                    backend.create_account(account_type, name, description).await.map(Some)
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

                    // TODO: handle future expected errors, e.g. duplicate account name
                    if let Ok(Some(account)) = res {
                        subview.pop();
                        on_exit.emit(Some(account.clone()));
                    }
                }
            }
        }),
    );

    let cancel_click = move |_| {
        subview.pop();
        on_exit.emit(None);
    };

    let create_click = move |_| create_account.run();

    html! {
        <div class="subview create-account">
            <div class="create-account-title">
                <h2>{"Create account"}</h2>
            </div>
            <div class="create-account-form">
                <SelectNullableEnum<AccountType>
                    state={account_type_state}
                    label="Account type"
                    required={true}
                    error={(*account_type_error_state).clone()}
                />
                <Input
                    state={account_name_state}
                    label="Account name"
                    on_submit={create_click.clone()}
                    required={true}
                    error={(*account_name_error_state).clone()}
                />
                <TextArea
                    state={account_description_state}
                    label="Account description"
                    error={(*account_description_error_state).clone()}
                />
            </div>
            <div class="create-account-actions">
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
