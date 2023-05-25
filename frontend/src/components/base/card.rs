use yew::prelude::*;

/// Card properties.
#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    /// Whether to show interactive animations.
    #[prop_or(false)]
    pub interactive: bool,
    /// The card click callback.
    #[prop_or(Callback::from(|_| ()))]
    pub on_click: Callback<()>,
    /// Elements within the card.
    pub children: Children,
}

/// A card component.
#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let CardProps {
        interactive,
        on_click,
        children,
    } = props.clone();

    let onclick = move |_| {
        on_click.emit(());
    };

    html! {
        <div
            {onclick}
            class={classes!("base-card", interactive.then_some("base-card-interactive"))}
        >
            <div class="base-card-inner">
                {children}
            </div>
        </div>
    }
}
