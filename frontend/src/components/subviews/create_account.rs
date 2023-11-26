use crate::components::base::*;
use crate::hooks::*;
use crate::view::View;
use yew::prelude::*;

/// The account creation subview.
#[function_component]
pub fn CreateAccount() -> Html {
    let view = use_view();
    let subview = use_subview();

    let go_back = move |_| {
        subview.pop();
    };
    let go_home = move |_| {
        view.set(View::Home);
    };

    html! {
        <div>
            <div>{"Create account placeholder"}</div>
            <Button text="Back" on_click={go_back}></Button>
            <Button text="Home" on_click={go_home}></Button>
        </div>
    }
}
