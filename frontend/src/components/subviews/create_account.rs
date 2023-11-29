use crate::components::base::*;
use crate::components::misc::*;
use crate::hooks::*;
use commands::FrontendCommands;
use common::*;
use yew::prelude::*;

/// Account creation subview properties.
#[derive(Clone, PartialEq, Properties)]
pub struct CreateAccountProps {
    /// The callback called when the subview is exited. The parameter passed
    /// to the function represents whether a new account was created.
    #[prop_or_default]
    pub on_exit: Callback<bool>,
}

/// The account creation subview.
#[function_component]
pub fn CreateAccount(props: &CreateAccountProps) -> Html {
    let CreateAccountProps { on_exit } = props.clone();

    let account_type_state = use_state(|| None);
    let account_type_error_state = use_state(|| None);
    let account_name_state = use_state(String::new);
    let account_description_state = use_state(String::new);
    let loading_state = use_state(|| false);

    let subview = use_subview();

    let create_account = use_command(
        UseCommand::new({
            let account_type_state = account_type_state.clone();
            let account_name_state = account_name_state.clone();
            let account_description_state = account_description_state.clone();
            |backend| async move {
                let account_type = (*account_type_state).unwrap(); // unwrap allowed because `None` case is being checked before the command is run
                let account_name = (*account_name_state).clone();
                let account_description = (*account_description_state).clone();
                backend
                    .create_account(account_type, account_name, account_description)
                    .await
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
                    if res.is_ok() {
                        subview.pop();
                        on_exit.emit(true);
                    }
                }
            }
        }),
    );

    let cancel_click = move |_| {
        subview.pop();
        on_exit.emit(false);
    };

    let create_click = {
        let account_type_state = account_type_state.clone();
        let account_type_error_state = account_type_error_state.clone();
        move |_| {
            if account_type_state.is_none() {
                account_type_error_state.set(Some("Please select an account type"));
            } else {
                create_account.run();
            }
        }
    };

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
                    error={*account_type_error_state}
                />
                <Input
                    state={account_name_state}
                    label="Account name"
                    on_submit={create_click.clone()}
                    required={true}
                />
                <TextArea
                    state={account_description_state}
                    label="Account description"
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
