use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Literal, TokenStream as TokenStream2, TokenTree};
use quote::quote;

#[derive(Clone, Debug)]
enum Node {
    Element(ElementNode),
    Expr(TokenStream2),
}

#[derive(Clone, Debug)]
struct ElementNode {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
}

#[derive(Clone, Debug)]
struct Attribute {
    name: String,
    value: AttributeValue,
}

#[derive(Clone, Debug)]
enum AttributeValue {
    Literal(Literal),
    Expr(TokenStream2),
}

struct Parser {
    tokens: Vec<TokenTree>,
    cursor: usize,
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let mut parser = Parser::new(TokenStream2::from(input));
    match parser.parse_root() {
        Ok(node) => generate_node(&node).into(),
        Err(message) => {
            let literal = Literal::string(&message);
            quote!(compile_error!(#literal);).into()
        }
    }
}

impl Parser {
    fn new(tokens: TokenStream2) -> Self {
        Self {
            tokens: tokens.into_iter().collect(),
            cursor: 0,
        }
    }

    fn parse_root(&mut self) -> Result<Node, String> {
        let node = self.parse_node()?;
        if self.cursor != self.tokens.len() {
            return Err("expected a single root node in rsx!".to_owned());
        }
        Ok(node)
    }

    fn parse_node(&mut self) -> Result<Node, String> {
        match self.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '<' => {
                Ok(Node::Element(self.parse_element()?))
            }
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                let stream = group.stream();
                self.cursor += 1;
                Ok(Node::Expr(stream))
            }
            _ => Err("expected an element like <main>...</main> or a {expr} child".to_owned()),
        }
    }

    fn parse_element(&mut self) -> Result<ElementNode, String> {
        self.expect_punct('<')?;
        let name = self.parse_name()?;

        let mut attributes = Vec::new();
        loop {
            if self.check_punct('/') && self.check_next_punct('>') {
                self.cursor += 2;
                return Ok(ElementNode {
                    name,
                    attributes,
                    children: Vec::new(),
                });
            }

            if self.check_punct('>') {
                self.cursor += 1;
                break;
            }

            attributes.push(self.parse_attribute()?);
        }

        let mut children = Vec::new();
        loop {
            if self.check_punct('<') && self.check_next_punct('/') {
                break;
            }
            children.push(self.parse_node()?);
        }

        self.expect_punct('<')?;
        self.expect_punct('/')?;
        let closing_name = self.parse_name()?;
        if closing_name != name {
            return Err(format!(
                "mismatched closing tag: expected </{}> but found </{}>",
                name, closing_name
            ));
        }
        self.expect_punct('>')?;

        Ok(ElementNode {
            name,
            attributes,
            children,
        })
    }

    fn parse_attribute(&mut self) -> Result<Attribute, String> {
        let name = self.parse_name()?;
        self.expect_punct('=')?;

        let value = match self.next() {
            Some(TokenTree::Literal(literal)) => AttributeValue::Literal(literal.clone()),
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                AttributeValue::Expr(group.stream())
            }
            _ => {
                return Err(format!(
                    "attribute {} must use either a string literal or a braced Rust expression",
                    name
                ))
            }
        };

        Ok(Attribute { name, value })
    }

    fn parse_name(&mut self) -> Result<String, String> {
        let mut name = match self.next() {
            Some(TokenTree::Ident(ident)) => ident.to_string(),
            _ => {
                return Err(format!(
                    "expected a tag or attribute name, found {:?} at token index {}",
                    self.peek(), self.cursor
                ))
            }
        };

        while self.check_punct('-') {
            self.cursor += 1;
            match self.next() {
                Some(TokenTree::Ident(ident)) => {
                    name.push('-');
                    name.push_str(&ident.to_string());
                }
                _ => return Err("expected an identifier after '-' in a tag or attribute name".to_owned()),
            }
        }

        Ok(name)
    }

    fn expect_punct(&mut self, expected: char) -> Result<(), String> {
        match self.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == expected => Ok(()),
            _ => Err(format!("expected '{}'", expected)),
        }
    }

    fn check_punct(&self, expected: char) -> bool {
        matches!(self.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == expected)
    }

    fn check_next_punct(&self, expected: char) -> bool {
        matches!(self.tokens.get(self.cursor + 1), Some(TokenTree::Punct(punct)) if punct.as_char() == expected)
    }

    fn peek(&self) -> Option<&TokenTree> {
        self.tokens.get(self.cursor)
    }

    fn next(&mut self) -> Option<&TokenTree> {
        let token = self.tokens.get(self.cursor);
        self.cursor += 1;
        token
    }
}

fn generate_node(node: &Node) -> TokenStream2 {
    match node {
        Node::Element(element) => generate_element(element),
        Node::Expr(expression) => {
            quote! {
                ::rust_fel::__private::text((#expression))
            }
        }
    }
}

fn generate_element(element: &ElementNode) -> TokenStream2 {
    let tag_name = Literal::string(&element.name);
    let attribute_setters: Vec<_> = element
        .attributes
        .iter()
        .map(generate_attribute)
        .collect();
    let children: Vec<_> = element.children.iter().map(generate_node).collect();

    quote! {{
        let mut props = ::rust_fel::Props::default();
        #(#attribute_setters)*
        {
            let children = ::std::vec![#(#children),*];
            if !children.is_empty() {
                props.children = Some(children);
            }
        }
        ::rust_fel::Element::new(#tag_name.to_owned(), props)
    }}
}

fn generate_attribute(attribute: &Attribute) -> TokenStream2 {
    let attribute_name = Literal::string(&attribute.name);

    match (&attribute.name[..], &attribute.value) {
        ("on_click", AttributeValue::Expr(expression)) => {
            quote! {
                props.on_click = Some(#expression);
            }
        }
        ("mouse", AttributeValue::Expr(expression)) => {
            quote! {
                props.mouse = Some(#expression);
            }
        }
        (_, AttributeValue::Literal(literal)) => {
            quote! {
                props.set_attribute(#attribute_name, #literal);
            }
        }
        (_, AttributeValue::Expr(expression)) => {
            quote! {
                props.set_attribute(#attribute_name, (#expression));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use quote::quote;

    #[test]
    fn parses_nested_angle_bracket_tree() {
        let tokens = quote! {
            <main id={"counter-app"} class="demo-app">
                <button type="button" data-cy="increment" on_click={click}>{"Increment"}</button>
            </main>
        };

        let mut parser = Parser::new(tokens);
        let result = parser.parse_root();

        assert!(result.is_ok(), "parser failed: {result:?}");
    }
}