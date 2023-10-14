use yew::prelude::*;

/// Save icon properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SaveIconProps {
    /// Is the save open?
    pub open: bool,
    /// The icon size in pixels.
    #[prop_or(32)]
    pub size: usize,
}

/// A save icon, in either an open or closed state.
#[function_component]
pub fn SaveIcon(props: &SaveIconProps) -> Html {
    let SaveIconProps { open, size } = props.clone();

    let icon_name = if open {
        "lock-open-solid"
    } else {
        "lock-solid"
    };
    let lock_icon = format!("assets/svg/{}.svg", icon_name);
    let outer_size = size;
    let inner_size = (size as f64) * 0.6;

    html! {
        <div class="save-icon">
            <img
                src="assets/svg/file-solid.svg"
                class="save-icon-file"
                style={format!("width: {}px; height: {}px;", outer_size, outer_size)}
            />
            <img
                src={lock_icon}
                class="save-icon-lock"
                style={format!("width: {}px; height: {}px;", inner_size, inner_size)}
            />
        </div>
    }
}
