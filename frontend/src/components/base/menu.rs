use super::*;
use crate::hooks::*;
use derive_more::From;
use std::rc::Rc;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;
use yew_hooks::use_click_away;

#[derive(Properties, PartialEq, Clone)]
pub struct MenuActionProps {
    /// The text on the menu action.
    pub text: AttrValue,
    /// The icon to display before the text.
    #[prop_or_default]
    pub icon: Option<AttrValue>,
    /// Whether the menu action is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// The menu action click callback.
    #[prop_or_default]
    pub on_click: Callback<()>,
}

#[function_component]
pub fn MenuAction(props: &MenuActionProps) -> Html {
    let MenuActionProps {
        text,
        icon,
        disabled,
        on_click,
    } = props.clone();

    let item_icon = icon
        .map(|icon| {
            html! {
                <div class="base-menu-item-icon">
                    <Icon
                        name={icon}
                        size={IconSize::Small}
                        {disabled}
                    />
                </div>
            }
        })
        .unwrap_or_default();

    let onclick = move |_| {
        if !disabled {
            on_click.emit(())
        }
    };

    html! {
        <div
            class={classes!("base-menu-item", disabled.then_some("base-menu-item-disabled"), "base-menu-item-action")}
            {onclick}
        >
            {item_icon}
            <div class="base-menu-item-text">{text}</div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MenuSubmenuProps {
    /// The callback called when the submenu open state changes.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// The callback called when an action has triggered in the menu or a
    /// submenu.
    #[prop_or_default]
    pub on_action: Callback<()>,
    /// The text on the submenu.
    pub text: AttrValue,
    /// The icon to display before the text.
    #[prop_or_default]
    pub icon: Option<AttrValue>,
    /// Whether the submenu is disabled.
    #[prop_or(false)]
    pub disabled: bool,
    /// Submenu elements.
    #[prop_or_default]
    pub children: ChildrenRenderer<MenuItem>,
}

#[function_component]
pub fn MenuSubmenu(props: &MenuSubmenuProps) -> Html {
    let MenuSubmenuProps {
        on_change,
        on_action,
        text,
        icon,
        disabled,
        children,
    } = props.clone();

    let submenu_state = use_state(|| false);

    let item_icon = icon
        .map(|icon| {
            html! {
                <div class="base-menu-item-icon">
                    <Icon
                        name={icon}
                        size={IconSize::Small}
                        {disabled}
                    />
                </div>
            }
        })
        .unwrap_or_default();

    let onmouseenter = {
        let submenu_state = submenu_state.clone();
        move |_| {
            if !disabled {
                submenu_state.set(true);
            }
        }
    };
    let onmouseleave = {
        let submenu_state = submenu_state.clone();
        move |_| {
            if !disabled {
                submenu_state.set(false);
            }
        }
    };

    html! {
        <div
            class={classes!("base-menu-item", disabled.then_some("base-menu-item-disabled"),"base-menu-item-submenu")}
            {onmouseenter}
            {onmouseleave}
        >
            {item_icon}
            <div class="base-menu-item-text">{text}</div>
            <div class="base-menu-item-arrow">
                <Icon
                    name="angle-right-solid"
                    size={IconSize::Small}
                    {disabled}
                />
            </div>
            <div class="base-menu-item-inner-menu-container">
                <div class="base-menu-item-inner-menu">
                    <Menu state={submenu_state} {on_change} {on_action}>
                        {children}
                    </Menu>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MenuSeparatorProps {}

#[function_component]
pub fn MenuSeparator(props: &MenuSeparatorProps) -> Html {
    let MenuSeparatorProps {} = props.clone();

    html! {
        <div class="base-menu-item base-menu-item-separator"></div>
    }
}

#[derive(Clone, PartialEq, From)]
pub enum MenuItem {
    Action(VChild<MenuAction>),
    Submenu(VChild<MenuSubmenu>),
    Separator(VChild<MenuSeparator>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for MenuItem {
    fn into(self) -> Html {
        match self {
            Self::Action(child) => child.into(),
            Self::Submenu(child) => child.into(),
            Self::Separator(child) => child.into(),
        }
    }
}

/// Menu properties.
#[derive(Properties, PartialEq, Clone)]
pub struct MenuProps {
    /// The menu open state.
    pub state: UseStateHandle<bool>,
    /// The callback called when the menu open state changes.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// The callback called when an action has triggered in the menu or a
    /// submenu.
    #[prop_or_default]
    pub on_action: Callback<()>,
    /// Elements within the menu.
    #[prop_or_default]
    pub children: ChildrenRenderer<MenuItem>,
}

/// A menu component.
#[function_component]
pub fn Menu(props: &MenuProps) -> Html {
    let MenuProps {
        state,
        on_change,
        on_action,
        children,
    } = props.clone();

    use_effect_with_deps(move |new_state| on_change.emit(**new_state), state.clone());

    let menu_node = use_node_ref();
    use_click_away(menu_node.clone(), {
        let state = state.clone();
        move |_| {
            state.set(false);
        }
    });

    let new_children = children
        .into_iter()
        .map(|child| match child {
            MenuItem::Action(mut action) => {
                let state = state.clone();
                let on_action = on_action.clone();
                let child_props = Rc::clone(&action.props);
                let mut new_props = (*action.props).clone();
                new_props.on_click = Callback::from(move |_| {
                    state.set(false);
                    child_props.on_click.emit(());
                    on_action.emit(());
                });
                action.props = Rc::new(new_props);
                MenuItem::from(action)
            }
            MenuItem::Submenu(mut submenu) => {
                let state = state.clone();
                let on_action = on_action.clone();
                let child_props = Rc::clone(&submenu.props);
                let mut new_props = (*submenu.props).clone();
                new_props.on_action = Callback::from(move |_| {
                    state.set(false);
                    child_props.on_action.emit(());
                    on_action.emit(());
                });
                submenu.props = Rc::new(new_props);
                MenuItem::from(submenu)
            }
            MenuItem::Separator(separator) => MenuItem::from(separator),
        })
        .collect::<Html>();

    let popup_node = use_node_ref();
    use_popup(popup_node.clone());

    html! {
        <div class={classes!("base-menu", state.then_some("base-menu-open"))} ref={menu_node}>
            <div ref={popup_node} class="base-menu-popup">
                {new_children}
            </div>
        </div>
    }
}
