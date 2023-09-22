use crate::util::*;
use gloo_timers::callback::Timeout;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// When applied to a popup element, this hook attempts to keep the element
/// within the viewing space on the screen.
#[hook]
pub fn use_popup(node: NodeRef) {
    let handler = Rc::new(move || {
        if let Some(node_inner) = node.get() {
            if let Ok(el) = node_inner.dyn_into::<HtmlElement>() {
                el.style()
                    .set_property("transform", "translate(0px, 0px)")
                    .unwrap();

                let doc = document();
                let doc_el = doc.document_element().unwrap();
                let win_width = doc_el.client_width() as f64;
                let win_height = doc_el.client_height() as f64;
                let el_rect = el.get_bounding_client_rect();
                let el_top = el_rect.top();
                let el_bottom = el_rect.bottom();
                let el_left = el_rect.left();
                let el_right = el_rect.right();

                let translate_x = if el_left < 0.0 {
                    -el_left
                } else if el_right > win_width {
                    win_width - el_right
                } else {
                    0.0
                };

                let translate_y = if el_top < 0.0 {
                    -el_top
                } else if el_bottom > win_height {
                    win_height - el_bottom
                } else {
                    0.0
                };

                el.style()
                    .set_property(
                        "transform",
                        &format!("translate({}px, {}px)", translate_x, translate_y),
                    )
                    .unwrap();
            }
        }
    });

    {
        let handler = handler.clone();
        use_event_with_window("resize", move |_: Event| {
            (*handler)();
        });
    }

    {
        let handler = handler.clone();
        use_event_with_window("scroll", move |_: Event| {
            (*handler)();
        });
    }

    Timeout::new(0, move || {
        (*handler)();
    })
    .forget();
}
