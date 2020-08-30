use crate::component::Component;
use crate::render::render;

/// ```App``` has a member, ```component```,  representing a [rust_fel::Element](../rust_fel/struct.Element.html).
/// implementing [rust_fel::Component](../rust_fel/trait.Component.html)
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
    /// ```App``` holds a [rust_fel::Element](../rust_fel/struct.Element.html) and  ```mount``` invokes the [rust_fel::Element's](../rust_fel/struct.Element.html) [render](../rust_fel/trait.Component.html#tymethod.render) function.
    /// After the [rust_fel::Element](../rust_fel/struct.Element.html) is created from the call to ```self.component.render();``` then ```rustfel::render::render``` is invoked.
    /// This constructs a Virtual [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction) and then the real [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction) in the browser.
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
