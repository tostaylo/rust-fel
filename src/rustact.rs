use crate::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub struct Element {
    html_type: String,
    props: Props,
}

pub struct Props {
    pub children: Option<Vec<Element>>,
    pub text: Option<String>,
    pub on_click: Option<Box<dyn FnMut() -> ()>>,
    pub class_name: Option<String>,
    pub id: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            children: None,
            text: None,
            on_click: None,
            class_name: None,
            id: None,
        }
    }
}

impl Element {
    pub fn new(html_type: String, props: Props) -> Element {
        Element { html_type, props }
    }
}

pub fn render(rustact_element: Element, container: &web_sys::Node) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if rustact_element.html_type == "TEXT_ELEMENT" {
        match rustact_element.props.text {
            Some(text) => {
                let dom = container
                    .append_child(&document.create_text_node(&text))
                    .expect("couldn't append text node");

                match rustact_element.props.children {
                    Some(children) => {
                        for child in children {
                            render(child, &dom)
                        }
                    }
                    None => (),
                }
            }
            None => (),
        };
    } else {
        let dom_el = document.create_element(&rustact_element.html_type).unwrap();

        match rustact_element.props.class_name {
            Some(class_name) => {
                dom_el.set_class_name(&class_name);
            }
            None => (),
        }

        match rustact_element.props.on_click {
            Some(mut on_click) => {
                let closure = Closure::wrap(Box::new(move || on_click()) as Box<dyn FnMut()>);
                dom_el
                    .dyn_ref::<HtmlElement>()
                    .expect("should be an `HtmlElement`")
                    .set_onclick(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }
            None => (),
        }

        let dom = container
            .append_child(&dom_el)
            .expect("couldn't append child");

        match rustact_element.props.children {
            Some(children) => {
                for child in children {
                    render(child, &dom)
                }
            }
            None => (),
        }
    }
}

pub fn create_element(html_type: String, props: Props) -> Element {
    Element::new(html_type, props)
}

pub type Reducer<T> = Box<dyn Fn(&T, &str) -> T>;
#[derive(Debug, Default, Clone, Copy)]
pub struct RustactStore<T> {
    store: T,
}

impl<T> RustactStore<T>
where
    T: std::fmt::Debug,
{
    pub fn new(store: T) -> Self {
        Self { store }
    }
    pub fn reduce(&mut self, reducer: Reducer<T>, action: &str) {
        let new_store = reducer(&self.store, action);
        self.store = new_store;
    }
    pub fn store(self) -> T {
        self.store
    }
}

pub fn re_render(app: Element) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let root = document
        .get_element_by_id("root")
        .expect("should have a root div");
    let node = root.first_child().unwrap();

    root.remove_child(&node).expect("unable to remove child");

    let root_node = root
        .append_child(&document.create_element("div").unwrap())
        .expect("couldn't append child");

    render(app, &root_node);
}
// pub fn use_reducer(initial_state: &'static State) -> (&State, Box<dyn FnMut(&str) -> ()>) {
//     let message_1 = format!("here is state initially {:?}", initial_state);
//     js::log(&message_1);
//     let mut state = initial_state;

//     let dispatch = Box::new(move |action: &str| {
//         state = &state.reduce(action);
//         let message_dispatch = format!("here is state in dispatch {:?}", state);

//         js::log(&message_dispatch);
//         if initial_state.order == false {
//             re_render();
//         }
//         ()
//     });
//     let message_2 = format!("here is state after dispatch {:?}", state);
//     js::log(&message_2);
//     (state, dispatch)
// }
// type UseState = (Rc<RefCell<i32>>, Box<dyn FnMut(i32, Element) -> ()>);
// pub fn rustact() -> Box<dyn FnMut(i32) -> UseState> {
//     let internal_state = Rc::new(RefCell::new(0));
//     let internal_state_copy = internal_state.clone();

//     let use_state = move |initial_state: i32| {
//         let val: i32;

//         if *internal_state_copy.borrow() > 0 {
//             val = *internal_state_copy.borrow();
//             log(&format!("{:?} setting val", internal_state));
//         } else {
//             *internal_state.borrow_mut() = initial_state;
//             val = initial_state;
//             log(&format!("{:?} setting internal", internal_state));
//         }

//         let state = Rc::new(RefCell::new(val));
//         let state_copy = state.clone();
//         let set_state = Box::new(move |new_val: i32, el: Element| {
//             *state_copy.borrow_mut() += new_val;
//         }) as Box<dyn FnMut(i32, Element) -> ()>;

//         (state, set_state)
//     };
//     return Box::new(use_state) as Box<dyn FnMut(i32) -> UseState>;
// }
pub fn parse_html(html_string: String) {
    // bytes instead?
    let mut arena_tree: ArenaTree<String> = ArenaTree::default();
    let tokens: Vec<char> = html_string.chars().collect();
    let mut html_type: Option<String> = None;
    let mut start_position = 0;
    let current_parent = 0;

    while start_position <= tokens.len() {
        if tokens[start_position].to_string() == "<" {
            let close_position = tokens.iter().position(|x| x.to_string() == ">").unwrap();
            let slice = &tokens[start_position + 1..close_position];
            html_type = Some(slice.into_iter().collect::<String>());
            start_position = close_position + 1;
            log(&format!("{:?}", html_type));

            arena_tree
                .arena
                .push(Node::new(0, html_type.unwrap(), None, None));
        } else {
            let rest_of_tokens = &tokens[start_position..];
            log(&format!("{:?} from else", &rest_of_tokens));
            let close_position = rest_of_tokens
                .iter()
                .position(|x| x.to_string() == "<")
                .unwrap();

            let slice = &rest_of_tokens[0..close_position];
            log(&format!("{:?} from else", slice));
            let val = Some(slice.into_iter().collect::<String>());
            arena_tree.arena.push(Node::new(
                current_parent + 1,
                val.unwrap(),
                Some(current_parent),
                None,
            ));

            //Let ArenaTree handle all the insertions and deletions instead of trying to manipulate the tree myself.
            let parent = &arena_tree.arena[current_parent];
            let children = &parent.children.unwrap();
            children.push(current_parent + 1);
            break;
        }
    }

    log(&format!("{:?}", arena_tree));
}
//Handle Siblings
//How to validate syntax?
//Parse and build AST
//Start tokenizing
//Look for first <
//Get first chars before either a space or a >
//Create node with parent and children indexes
//Children = first char after first >
//Get all children and store their indexes.

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn new(arena: Vec<Node<T>>) -> Self {
        Self { arena }
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Option<Vec<usize>>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T, parent: Option<usize>, children: Option<Vec<usize>>) -> Self {
        Self {
            idx,
            val,
            parent,
            children,
        }
    }
}
