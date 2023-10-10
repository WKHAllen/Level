use yew::prelude::*;

/// The lightness of the frame background.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FrameBackground {
    BG1,
    BG2,
    BG3,
    #[default]
    BG4,
    BG5,
    BG6,
}

impl FrameBackground {
    /// Gets the index of the frame background lightness.
    pub fn index(&self) -> usize {
        *self as usize + 1
    }
}

/// Frame properties.
#[derive(Properties, PartialEq, Clone)]
pub struct FrameProps {
    /// The frame background lightness.
    #[prop_or_default]
    pub background: FrameBackground,
    /// Classes to apply to the frame.
    #[prop_or_default]
    pub class: Classes,
    /// Child elements.
    #[prop_or_default]
    pub children: Children,
}

/// A basic UI frame.
#[function_component]
pub fn Frame(props: &FrameProps) -> Html {
    let FrameProps {
        background,
        mut class,
        children,
    } = props.clone();

    class.push("base-frame");
    class.push(format!("base-frame-background-{}", background.index()));

    html! {
        <div {class}>
            {children}
        </div>
    }
}
