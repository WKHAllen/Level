use crate::hooks::*;
use commands::FrontendCommands;
use yew::prelude::*;

/// A random quote from the database, and a button to fetch a new one.
#[function_component]
pub fn Quote() -> Html {
    let (backend, _) = use_backend();

    let quote = use_async(
        async move { Result::<_, ()>::Ok(backend.get_random_quote().await) },
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
