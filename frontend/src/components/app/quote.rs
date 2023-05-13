use crate::hooks::*;
use crate::state::State;
use commands::FrontendCommands;
use yew::prelude::*;
use yewdux::prelude::*;

/// A random quote from the database, and a button to fetch a new one.
#[function_component]
pub fn Quote() -> Html {
    let (state, _) = use_store::<State>();

    let quote = use_async(
        async move { Result::<_, ()>::Ok(state.get_random_quote().await) },
        true,
    );

    let onclick = {
        let quote = quote.clone();
        move |_| {
            quote.run();
        }
    };

    html! {
        <div>
            {
                match &*quote {
                    UseAsyncState::Init => html! { <p>{ "Initializing..." }</p> },
                    UseAsyncState::Loading(prev) => match prev {
                        PreviousUseAsyncState::None => html! { <p>{ "Fetching quote..." }</p> },
                        PreviousUseAsyncState::Success(data) => html! { <p>{ data }</p> },
                        PreviousUseAsyncState::Failure(_) => unreachable!(),
                    },
                    UseAsyncState::Success(data) => html! { <p>{ data }</p> },
                    UseAsyncState::Failure(_) => unreachable!(),
                }
            }
            <button {onclick} disabled={quote.loading()}>{ "New quote" }</button>
        </div>
    }
}
