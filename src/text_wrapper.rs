use crate::rustact;

pub fn text_wrapper(
    html_type: String,
    children: Option<Vec<rustact::Element>>,
    on_click: Option<rustact::ClosureProp>,
    class_name: Option<String>,
) -> rustact::Element {
    let text_container = rustact::create_element(
        html_type,
        rustact::Props {
            on_click,
            class_name,
            children,
            ..Default::default()
        },
    );

    text_container
}
