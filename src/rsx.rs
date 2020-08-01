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
  let mut stack: Vec<StackElement> = vec![];
  let mut arena_tree: ArenaTree = ArenaTree::default();

  //if stack has a length we have one parent.
  while let Some(character) = tokens.next() {
    let string_character = character.to_string();

    if string_character == "<" && tokens.peek().unwrap().to_string() != "/" {
      is_open_tag = true;
      continue;
    }
    if string_character == "<" && tokens.peek().unwrap().to_string() == "/" {
      is_open_tag = false;
      stack.pop();

      continue;
    }

    if string_character == ">" {
      if element_type != "".to_string() {
        //Let's go back to dealing with Options instead of empty strings.

        let el = element_type.clone();
        if stack.len() >= 1 {
          arena_tree.set_current_parent_idx(stack.last().unwrap().arena_position);
        } else {
          arena_tree.set_current_parent_idx(0);
        }

        arena_tree.insert(Node {
          element_type,
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

    if is_open_tag == true {
      element_type.push_str(&string_character);
    }
  }

  arena_tree
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
      let new_el = Element {
        html_type: node.element_type.clone(),
        props: Props {
          children: children(node, arena),
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
}

impl Node {
  fn add_child(&mut self, child_idx: usize) {
    self.children.push(child_idx);
  }
}
