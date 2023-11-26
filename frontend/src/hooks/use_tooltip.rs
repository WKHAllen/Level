use crate::util::*;
use gloo_timers::callback::Timeout;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// Creates a tooltip element.
#[hook]
pub fn use_tooltip(node: NodeRef, text: &str) {
    let text = text.to_owned();
    let tooltip_node_state = use_state(|| None);

    // use_popup()

    let handler = Rc::new({
        let node = node.clone();
        let tooltip_node_state = tooltip_node_state.clone();
        move || {
            if tooltip_node_state.is_none() {
                if let Some(node_inner) = node.get() {
                    if let Ok(el) = node_inner.dyn_into::<HtmlElement>() {
                        let doc = document();
                        let tooltip_container_el = doc.create_element("div").unwrap();
                        tooltip_container_el.set_class_name("tooltip-container");
                        let tooltip_el = doc.create_element("div").unwrap();
                        tooltip_el.set_class_name("tooltip");
                        let tooltip_text_container_el = doc.create_element("div").unwrap();
                        tooltip_text_container_el.set_class_name("tooltip-text-container");
                        let tooltip_text_el = doc.create_element("span").unwrap();
                        tooltip_text_el.set_class_name("tooltip-text");
                        tooltip_text_el.set_text_content(Some(&text));
                        tooltip_text_container_el
                            .append_child(&tooltip_text_el)
                            .unwrap();
                        tooltip_el.append_child(&tooltip_text_container_el).unwrap();
                        tooltip_container_el.append_child(&tooltip_el).unwrap();
                        el.append_child(&tooltip_container_el).unwrap();
                        tooltip_node_state.set(Some(tooltip_el));
                    }
                }
            }
        }
    });

    use_event(node.clone(), "mouseover", {
        let tooltip_node_state = tooltip_node_state.clone();
        move |_: MouseEvent| {
            if let Some(tooltip_el) = &*tooltip_node_state {
                tooltip_el.set_class_name("tooltip tooltip-visible");
            }
        }
    });

    use_event(node, "mouseout", move |_: MouseEvent| {
        if let Some(tooltip_el) = &*tooltip_node_state {
            tooltip_el.set_class_name("tooltip");
        }
    });

    Timeout::new(0, move || {
        (*handler)();
    })
    .forget();
}
