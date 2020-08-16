use crate::element::Element;
use crate::props::{ClosureProp, Props};

/// This is testable and documentable
pub fn wrapper(
    html_type: String,
    text: Option<String>,
    on_click: Option<ClosureProp>,
    class_name: Option<String>,
    elements: Option<Element>,
) -> Element {
    let children;

    match (elements, text) {
        (Some(el), None) => {
            children = Some(vec![el]);
        }
        (None, Some(t)) => {
            let text_el = Element::new(
                "TEXT_ELEMENT".to_owned(),
                Props {
                    text: Some(format!("{}", t)),
                    ..Default::default()
                },
            );
            children = Some(vec![text_el]);
        }
        _ => panic!("Have to have at least (text) or (elements) but not both."),
    };

    Element::new(
        html_type,
        Props {
            on_click,
            class_name,
            children,
            ..Default::default()
        },
    )
}
