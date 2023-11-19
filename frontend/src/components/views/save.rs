use crate::components::misc::*;
use yew::prelude::*;

/// The open save page view.
#[function_component]
pub fn Save() -> Html {
    let info_pane_state = use_state(|| true);
    let stats_pane_state = use_state(|| false);
    let answers_pane_state = use_state(|| false);

    html! {
        <div class="view save">
            <div class="save-header bg-4">
                <span>{"Save name"}</span>
            </div>
            <div class="save-body">
                <div class="save-left bg-3">
                    <div class="save-accounts">
                        <span>{"Accounts"}</span>
                    </div>
                </div>
                <div class="save-main bg-2">
                    <div class="account-transactions">
                        <span>{"Transactions"}</span>
                    </div>
                </div>
                <div class="save-right bg-3">
                    <div class="account-info">
                        <ExpandablePane state={info_pane_state} label="Info">
                            <span>{"Info pane"}</span>
                        </ExpandablePane>
                        <ExpandablePane state={stats_pane_state} label="Stats">
                            <div>{"Stats pane"}</div>
                        </ExpandablePane>
                        <ExpandablePane state={answers_pane_state} label="Answers">
                            <span>{"Answers pane"}</span>
                        </ExpandablePane>
                    </div>
                </div>
            </div>
        </div>
    }
}
