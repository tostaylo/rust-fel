use crate::element::Element;
use crate::props::Props;

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

  //if stack has a length we have one parent.
  while let Some(character) = tokens.next() {
    let string_character = character.to_string();

    if string_character == "<" {
      if has_text {
        // This pattern may need to be function-ized since we may need to repeat.
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

    if string_character == ">" {
      if element_type != "".to_string() {
        let next_token = tokens.peek().unwrap().to_string();
        if next_token != "<" {
          has_text = true;
        };
        let el = element_type.clone();
        if stack.len() >= 1 {
          arena_tree.set_current_parent_idx(stack.last().unwrap().arena_position);
        } else {
          arena_tree.set_current_parent_idx(0);
        }

        arena_tree.insert(Node {
          element_type: element_type.clone(),
          ..Default::default()
        });
        stack.push(StackElement {
          val: el,
          arena_position: arena_tree.arena.len() - 1,
        });

        element_type = String::new();
      }
      continue;
    }

    // Down here we decide what types of variables to push to

    if is_open_tag == true && has_text == false {
      element_type.push_str(&string_character);
      continue;
    }
    if has_text {
      text.push_str(&string_character);
      continue;
    }
  }
  arena_tree
}

#[cfg(test)]
#[test]
pub fn is_parent_correct() {
  let arena1 = parse_with_stack("<div><div>here is some text</div></div>".to_owned());
  println!("{:?}", arena1);
  assert_eq!(arena1.arena[2].parent, 1);
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
      let new_el = Element {
        html_type: node.element_type.clone(),
        props: Props {
          children: children(node, arena),
          text,
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

#[derive(Debug, Default)]
struct Node {
  idx: usize,
  element_type: String,
  parent: usize,
  children: Vec<usize>,
  text: Option<String>,
}

impl Node {
  fn add_child(&mut self, child_idx: usize) {
    self.children.push(child_idx);
  }
}
