use crate::element::Element;
use crate::props::Props;
use std::fmt;

#[derive(Debug, Default, Clone)]
struct StackElement {
  val: String,
  arena_position: usize,
}

// Can I make a parser struct that's not coupled to arena tree?
// Input has to have a wrapper div.
pub fn parse_with_stack(html_string: String) -> ArenaTree {
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
        if stack.len() >= 1 {
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
      if element_type != "".to_string() {
        let next_token = tokens.peek().unwrap().to_string();

        // Here we must have text if it's not another element
        if next_token != "<" {
          has_text = true;
        };

        // Here we insert our element type and all attributes collected
        let el = element_type.clone();
        if stack.len() >= 1 {
          arena_tree.set_current_parent_idx(stack.last().unwrap().arena_position);
        } else {
          arena_tree.set_current_parent_idx(0);
        }

        let mut class_name = None;
        if attributes.len() >= 1 {
          let attributes_split = attributes.split(" ").filter(|s| !s.is_empty());
          for attribute in attributes_split {
            let attr = attribute.split("=").collect::<Vec<&str>>();
            match attr[0] {
              "class" => class_name = Some(attr[1].to_owned()),
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

    if string_character == "|" && has_attributes == false {
      has_attributes = true;
      continue;
    }
    // Down here we decide what types of variables to push to
    if is_open_tag == true && has_text == false {
      if string_character != " ".to_owned() && has_attributes == false {
        element_type.push_str(&string_character);
        continue;
      }

      if has_attributes == true && string_character != "|" {
        attributes.push_str(&string_character);
        continue;
      }
    }

    if has_text {
      text.push_str(&string_character);
      continue;
    }
  }

  if stack.len() > 0 {
    panic!("Your HTML is not formed correctly");
  }

  arena_tree
}

#[cfg(test)]
#[test]
pub fn is_parent_correct() {
  let arena_tree =
    parse_with_stack("<div |class=classname|><div>here is some text</div></div>".to_owned());
  assert_eq!(arena_tree.arena[2].parent, 1);
  let arena_tree =
    parse_with_stack("<div><div>here is some text</div><span></span></div>".to_owned());
  assert_eq!(arena_tree.arena[2].parent, 1);
  let arena_tree =
    parse_with_stack("<div><div><span>here is some text</span></div></div>".to_owned());
  assert_eq!(arena_tree.arena[3].parent, 2);
  let arena_tree = parse_with_stack("<div>Hi there</div>".to_owned());
  assert_eq!(arena_tree.arena[1].parent, 0);
}

#[cfg(test)]
#[test]
#[should_panic(expected = "Your HTML is not formed correctly")]
pub fn is_correct_html() {
  parse_with_stack("<div |class=classname|><div>here is some text</div>".to_owned());
  parse_with_stack("<div><div>here is some text<div></div>".to_owned());
}

#[cfg(test)]
#[test]

pub fn is_correct_attributes() {
  let arena_tree = parse_with_stack(
    "<div |class=classname on_click=onclick|><div>here is some text</div></div>".to_owned(),
  );
  assert_eq!(
    arena_tree.arena[0].class_name.as_ref().unwrap(),
    &"classname".to_owned()
  );
}

pub fn html(html_string: String) -> Element {
  let arena_tree = parse_with_stack(html_string);
  let el = arena_tree.create_element_from_tree();
  el
}

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
}

trait CreateElement {
  fn create_element_from_tree(&self) -> Element;
}

impl CreateElement for ArenaTree {
  fn create_element_from_tree(&self) -> Element {
    let arena = &self.arena;

    fn children(node: &Node, arena: &Vec<Node>) -> Option<Vec<Element>> {
      return Some(
        node
          .children
          .iter()
          .map(|child| return create(&arena[child.to_owned()], &arena))
          .collect::<Vec<Element>>(),
      );
    };

    fn create(node: &Node, arena: &Vec<Node>) -> Element {
      let text = match &node.text {
        Some(x) => Some(x.to_owned()),
        None => None,
      };

      let class_name = match &node.class_name {
        Some(x) => Some(x.to_owned()),
        None => None,
      };
      let new_el = Element {
        html_type: node.element_type.clone(),
        props: Props {
          children: children(node, arena),
          text,
          class_name,
          ..Default::default()
        },
      };

      new_el
    }
    let node = &arena[0];
    let el = create(node, arena);

    el
  }
}

#[derive(Default)]
struct Node {
  idx: usize,
  element_type: String,
  parent: usize,
  children: Vec<usize>,
  text: Option<String>,
  class_name: Option<String>,
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
