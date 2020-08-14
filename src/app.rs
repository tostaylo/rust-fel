use crate::component::Component;
use crate::render::render;
#[derive(Debug)]
pub struct App<Component> {
    component: Component,
}

impl<COMPONENT> App<COMPONENT>
where
    COMPONENT: Component,
    COMPONENT::Properties: Default,
    COMPONENT: std::fmt::Debug,
{
    pub fn new(component: COMPONENT) -> Self {
        App { component }
    }

    pub fn mount(&self, node_id: &str) {
        let el = self.component.render();
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let root_node = document
            .get_element_by_id(node_id)
            .expect("should have a root div");

        render(el, &root_node, false);
    }
}
