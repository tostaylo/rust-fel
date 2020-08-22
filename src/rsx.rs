use crate::element::Element;
use crate::props::Props;
use std::fmt;

#[derive(Debug, Default, Clone)]
struct StackElement {
    val: String,
    arena_position: usize,
}

/// Takes a string which is formatted in an HTML-like structure.
/// Parses the string contents and builds a [rust_fel::ArenaTree](../rsx/struct.ArenaTree.html)
/// # Arguments
///
/// * `html_string` - Must have a parent wrapping html element. All text must have a wrapping element. Text and non text elements cannot be siblings.
///
/// # Examples
///```ignore
///   // <div></div> <div></div> will not work.
///   // <div><div></div></div> will work.
///   // <div> Hi <span>Hello</span></div> will not work.
///   // <div> </span>Hi</span><span>Hello</span></div> will work.
///
///   let arena_tree =
///       parse_html_to_arena_tree("<div |class=classname|><div>here is some text</div></div>".to_owned());
///       assert_eq!(arena_tree.arena[2].parent, 1);
///   let arena_tree =
///      parse_html_to_arena_tree("<div><div><span>here is some text</span></div></div>".to_owned());
///      assert_eq!(arena_tree.arena[3].parent, 2);
///```

pub fn parse_html_to_arena_tree(html_string: String) -> ArenaTree {
    let mut tokens = html_string.chars().peekable();
    let mut element_type: String = String::new();
    let mut is_open_tag: bool = false;
    let mut has_text: bool = false;
    let mut text: String = String::new();
    let mut stack: Vec<StackElement> = vec![];
    let mut arena_tree: ArenaTree = ArenaTree::default();
    let mut has_attributes: bool = false;
    let mut attributes: String = String::new();

    while let Some(character) = tokens.next() {
        let string_character = character.to_string();

        if string_character == "<" {
            if has_text {
                // This pattern may need to be function-ized since we may need to repeat.
                // If the child is a text elment we insert
                // We need to let arena tree know the positions of the parent
                // In arenatree.insert we set the parent's and the children of the element being inserted
                if !stack.is_empty() {
                    arena_tree.set_current_parent_idx(stack.last().unwrap().arena_position);
                } else {
                    arena_tree.set_current_parent_idx(0);
                }
                arena_tree.insert(Node {
                    element_type: "TEXT_ELEMENT".to_owned(),
                    text: Some(text.clone()),
                    ..Default::default()
                });
            }
            if tokens.peek().unwrap().to_string() != "/" {
                is_open_tag = true;
            }
            if tokens.peek().unwrap().to_string() == "/" {
                is_open_tag = false;
                stack.pop();
            }

            has_text = false;
            text = "".to_owned();
            continue;
        }

        // Either last element of string or there will be a text child or another element
        if string_character == ">" {
            if element_type != "" {
                let next_token = tokens.peek().unwrap().to_string();

                // Here we must have text if it's not another element
                if next_token != "<" {
                    has_text = true;
                };

                // Here we insert our element type and all attributes collected
                let el = element_type.clone();
                if !stack.is_empty() {
                    arena_tree.set_current_parent_idx(stack.last().unwrap().arena_position);
                } else {
                    arena_tree.set_current_parent_idx(0);
                }

                let mut class_name = None;
                let mut href = None;
                let mut id = None;
                let mut src = None;
                let mut role = None;
                let mut type_attr = None;
                let mut data_cy = None;
                if !attributes.is_empty() {
                    let attributes_split = attributes.split(' ').filter(|s| !s.is_empty());
                    for attribute in attributes_split {
                        let attr = attribute.split('=').collect::<Vec<&str>>();
                        match attr[0] {
                            "class" => class_name = Some(attr[1].to_owned()),
                            "href" => href = Some(attr[1].to_owned()),
                            "src" => src = Some(attr[1].to_owned()),
                            "role" => role = Some(attr[1].to_owned()),
                            "type" => type_attr = Some(attr[1].to_owned()),
                            "id" => id = Some(attr[1].to_owned()),
                            "data-cy" => data_cy = Some(attr[1].to_owned()),
                            // Try Rc<RefCell>attribute handlers at the top of this function.
                            // match attribute handlers borrow_mut()
                            // "on_click" => {
                            //   match attribute_handlers {
                            //     Some(the_vec) => {
                            //       let idx = attr[1].to_owned().parse::<usize>().unwrap();
                            //       let handler = the_vec.into_iter().nth(idx).unwrap();
                            //       on_click = Some(handler);
                            //     }
                            //     None => (),
                            //   };
                            // }
                            _ => (),
                        };
                    }
                }
                arena_tree.insert(Node {
                    element_type: element_type.clone(),
                    class_name,
                    href,
                    data_cy,
                    id,
                    src,
                    role,
                    type_attr,
                    ..Default::default()
                });
                stack.push(StackElement {
                    val: el,
                    arena_position: arena_tree.arena.len() - 1,
                });
                // Reset everything so we have no data and can reade child element
                element_type = String::new();
                attributes = String::new();
                has_attributes = false;
            }
            continue;
        }

        if string_character == "|" && !has_attributes {
            has_attributes = true;
            continue;
        }
        // Down here we decide what types of variables to push to
        if is_open_tag && !has_text {
            if string_character != " " && !has_attributes {
                element_type.push_str(&string_character);
                continue;
            }

            if has_attributes && string_character != "|" {
                attributes.push_str(&string_character);
                continue;
            }
        }

        if has_text {
            text.push_str(&string_character);
            continue;
        }
    }

    if !stack.is_empty() {
        panic!("Your HTML is not formed correctly");
    }

    arena_tree
}

#[cfg(test)]
#[test]
pub fn is_parent_correct() {
    let arena_tree = parse_html_to_arena_tree(
        "<div |class=classname|><div>here is some text</div></div>".to_owned(),
    );
    assert_eq!(arena_tree.arena[2].parent, 1);
    let arena_tree =
        parse_html_to_arena_tree("<div><div>here is some text</div><span></span></div>".to_owned());
    assert_eq!(arena_tree.arena[2].parent, 1);
    let arena_tree =
        parse_html_to_arena_tree("<div><div><span>here is some text</span></div></div>".to_owned());
    assert_eq!(arena_tree.arena[3].parent, 2);
    let arena_tree = parse_html_to_arena_tree("<div>Hi there</div>".to_owned());
    assert_eq!(arena_tree.arena[1].parent, 0);
}

#[cfg(test)]
#[test]
#[should_panic(expected = "Your HTML is not formed correctly")]
pub fn is_correct_html() {
    parse_html_to_arena_tree("<div |class=classname|><div>here is some text</div>".to_owned());
    parse_html_to_arena_tree("<div><div>here is some text<div></div>".to_owned());
}

#[cfg(test)]
#[test]
pub fn is_correct_attributes() {
    let arena_tree = parse_html_to_arena_tree(
        "<div |class=classname href=https://www.google.com |><div |class=hi href=https://www.googles.com |>here is some text</div></div>"
            .to_owned(),
    );
    assert_eq!(
        arena_tree.arena[0].class_name.as_ref().unwrap(),
        &"classname".to_owned()
    );
    assert_eq!(
        arena_tree.arena[0].href.as_ref().unwrap(),
        &"https://www.google.com".to_owned()
    );
    assert_eq!(
        arena_tree.arena[1].class_name.as_ref().unwrap(),
        &"hi".to_owned()
    );
    assert_eq!(
        arena_tree.arena[1].href.as_ref().unwrap(),
        &"https://www.googles.com".to_owned()
    );
    assert_ne!(
        arena_tree.arena[1].href.as_ref().unwrap(),
        &"https://www.google.com".to_owned()
    );
    let arena_tree =
        parse_html_to_arena_tree("<script |src=https://www.google.com |></script>".to_owned());
    assert_eq!(
        arena_tree.arena[0].src.as_ref().unwrap(),
        &"https://www.google.com".to_owned()
    );

    let arena_tree = parse_html_to_arena_tree("<button | type=button role=button |><button | type=button role=button |></button></button>".to_owned());
    assert_eq!(
        arena_tree.arena[0].type_attr.as_ref().unwrap(),
        &"button".to_owned()
    );
    assert_eq!(
        arena_tree.arena[0].role.as_ref().unwrap(),
        &"button".to_owned()
    );
    assert_eq!(
        arena_tree.arena[1].type_attr.as_ref().unwrap(),
        &"button".to_owned()
    );
    assert_eq!(
        arena_tree.arena[1].role.as_ref().unwrap(),
        &"button".to_owned()
    );
    let arena_tree = parse_html_to_arena_tree("<button | data-cy=cypress role=button |><button | data-cy=cypress type=button role=button |></button></button>".to_owned());
    assert_eq!(
        arena_tree.arena[0].data_cy.as_ref().unwrap(),
        &"cypress".to_owned()
    );
    assert_eq!(
        arena_tree.arena[1].data_cy.as_ref().unwrap(),
        &"cypress".to_owned()
    );
}

/// parse html string to create a virtual dom
pub fn html(html_string: String) -> Element {
    let arena_tree = parse_html_to_arena_tree(html_string);
    arena_tree.create_element_from_tree()
}

#[cfg(test)]
#[test]
pub fn creates_html() {
    let html = html(
        "<div |class=classname|><span |role=button|>Span Text</span><p>Paragraph Text</p></div>"
            .to_owned(),
    );
    assert_eq!(html.html_type, "div".to_owned());
    assert_eq!(html.props.class_name.unwrap(), "classname".to_owned());

    let children = html.props.children.unwrap();
    let first_child = children.iter().nth(0);
    let second_child = children.iter().nth(1);
    assert_eq!(first_child.unwrap().html_type, "span");
    assert_eq!(first_child.unwrap().props.role.as_ref().unwrap(), "button");
    assert_eq!(second_child.unwrap().html_type, "p");

    let second_childs_child = &second_child
        .unwrap()
        .props
        .children
        .iter()
        .nth(0)
        .unwrap()
        .iter()
        .nth(0)
        .unwrap();
    assert_eq!(second_childs_child.html_type, "TEXT_ELEMENT");
}

/// A structure which builds an ```arena``` ([std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)) of [rust_fel::Node](../rsx/struct.Node.html)'s that represent a tree structure.
/// # Examples
/// ```ignore
///   arena_tree.insert(Node {
///     element_type: element_type.clone(),
///     class_name,
///     href,
///     data_cy,
///     id,
///     src,
///     role,
///     type_attr,
///     ..Default::default()
///     });
/// ```
#[derive(Debug, Default)]
pub struct ArenaTree {
    current_parent_idx: usize,
    arena: Vec<Node>,
}

impl ArenaTree {
    fn set_current_parent_idx(&mut self, idx: usize) {
        self.current_parent_idx = idx;
    }

    fn insert(&mut self, mut node: Node) {
        node.parent = self.current_parent_idx;
        node.idx = self.arena.len();
        self.arena.push(node);
        let child_index = self.arena.len() - 1;
        let parent_node = &mut self.arena[self.current_parent_idx];
        if child_index > 0 {
            parent_node.add_child(child_index);
        }
    }

    fn create_element_from_tree(&self) -> Element {
        let arena = &self.arena;

        fn children(node: &Node, arena: &[Node]) -> Option<Vec<Element>> {
            Some(
                node.children
                    .iter()
                    .map(|child| create(&arena[child.to_owned()], &arena))
                    .collect::<Vec<Element>>(),
            )
        };

        fn create(node: &Node, arena: &[Node]) -> Element {
            let text = match &node.text {
                Some(x) => Some(x.to_owned()),
                None => None,
            };

            let class_name = match &node.class_name {
                Some(x) => Some(x.to_owned()),
                None => None,
            };

            let href = match &node.href {
                Some(x) => Some(x.to_owned()),
                None => None,
            };

            let data_cy = match &node.data_cy {
                Some(x) => Some(x.to_owned()),
                None => None,
            };

            let id = match &node.id {
                Some(x) => Some(x.to_owned()),
                None => None,
            };

            let src = match &node.src {
                Some(x) => Some(x.to_owned()),
                None => None,
            };

            let type_attr = match &node.type_attr {
                Some(x) => Some(x.to_owned()),
                None => None,
            };

            let role = match &node.role {
                Some(x) => Some(x.to_owned()),
                None => None,
            };

            Element {
                html_type: node.element_type.clone(),
                props: Props {
                    children: children(node, arena),
                    text,
                    class_name,
                    href,
                    data_cy,
                    id,
                    src,
                    type_attr,
                    role,
                    ..Default::default()
                },
            }
        }
        let node = &arena[0];
        create(node, arena)
    }
}
/// A ```Node``` is an intermediary representation of an HTML element.
/// A ```Node``` is constructed as a result of a html string being parsed. It will be inserted into a arena tree after initialization.
/// # Examples
/// ```ignore
///   arena_tree.insert(Node {
///     element_type: element_type.clone(),
///     class_name,
///     href,
///     data_cy,
///     id,
///     src,
///     role,
///     type_attr,
///     ..Default::default()
///     });
/// ```

#[derive(Default)]
pub struct Node {
    idx: usize,
    element_type: String,
    parent: usize,
    children: Vec<usize>,
    text: Option<String>,
    id: Option<String>,
    class_name: Option<String>,
    href: Option<String>,
    src: Option<String>,
    type_attr: Option<String>,
    role: Option<String>,
    data_cy: Option<String>,
    // on_click: Option<ClosureProp>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#?}, {:#?} this is a node",
            self.element_type, self.class_name
        )
    }
}

impl Node {
    fn add_child(&mut self, child_idx: usize) {
        self.children.push(child_idx);
    }
}
