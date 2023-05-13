use crate::hooks::*;
use crate::state::State;
use commands::FrontendCommands;
use yew::prelude::*;
use yewdux::prelude::*;

/// Greeting properties.
#[derive(Properties, PartialEq, Clone)]
pub struct GreetingProps {
    /// The name of the person to greet.
    #[prop_or("<unknown>".to_owned())]
    pub name: String,
}

/// A personal greeting.
#[function_component]
pub fn Greeting(props: &GreetingProps) -> Html {
    let (state1, _) = use_store::<State>();
    let state2 = state1.clone();

    let name = props.name.clone();

    use_async(
        async move { Result::<_, ()>::Ok(state1.say_hi().await) },
        true,
    );
    let greeting = use_async(
        async move { Result::<_, ()>::Ok(state2.greet(name).await) },
        true,
    );

    match &*greeting {
        UseAsyncState::Init | UseAsyncState::Loading(_) => {
            html! { <p>{ "Fetching greeting..." }</p> }
        }
        UseAsyncState::Success(g) => html! { <p>{g}</p> },
        UseAsyncState::Failure(_) => unreachable!(),
    }
}
