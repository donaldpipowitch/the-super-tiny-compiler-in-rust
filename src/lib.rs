use std::iter::Peekable;
use std::vec::IntoIter;
use std::collections::HashMap;

#[derive(Debug,PartialEq,Clone)]
pub enum Token {
    Number(String),
    String(String),
    Name(String),
    ParenOpening,
    ParenClosing,
}

pub fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();
    while let Some(c) = char_iter.next() {
        match c {
            c if c.is_whitespace() => continue,
            '(' => tokens.push(Token::ParenOpening),
            ')' => tokens.push(Token::ParenClosing),
            '0'...'9' => {
                let mut value = String::new();
                value.push(c);

                while let Some(&'0'...'9') = char_iter.peek() {
                    value.push(char_iter.next().unwrap());
                }

                tokens.push(Token::Number(value));
            }
            'a'...'z' => {
                let mut value = String::new();
                value.push(c);

                while let Some(&'a'...'z') = char_iter.peek() {
                    value.push(char_iter.next().unwrap());
                }

                tokens.push(Token::Name(value));
            }
            '"' => {
                let mut value = String::new();
                // ignore opening "

                while match char_iter.peek() {
                          Some(&'"') | None => false,
                          _ => true,
                      } {
                    value.push(char_iter.next().unwrap());
                }
                tokens.push(Token::String(value));

                // skip closing ""
                char_iter.next().unwrap();
            }
            _ => return Err(format!("I don't know what this character is: {}", c)),
        }
    }

    Ok(tokens)
}

#[derive(Debug,PartialEq,Eq,Hash)]
pub enum NodeType {
    Program,
    CallExpression,
    StringLiteral,
    NumberLiteral,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Node {
    Program { body: Vec<Node> },
    CallExpression { name: String, params: Vec<Node> },
    StringLiteral(String),
    NumberLiteral(String),
}

impl Node {
    fn get_type(&self) -> NodeType {
        match *self {
            Node::Program { .. } => NodeType::Program,
            Node::CallExpression { .. } => NodeType::CallExpression,
            Node::StringLiteral(_) => NodeType::StringLiteral,
            Node::NumberLiteral(_) => NodeType::NumberLiteral,
        }
    }
}

pub fn parser(tokens: Vec<Token>) -> Result<Node, String> {
    fn walk(token: Token, token_iter: &mut Peekable<IntoIter<Token>>) -> Result<Node, String> {
        match token {
            Token::Number(value) => Ok(Node::NumberLiteral(value)),
            Token::String(value) => Ok(Node::StringLiteral(value)),
            Token::ParenOpening => {
                if let Some(token) = token_iter.next() {
                    match token {
                        Token::Name(name) => {
                            let mut params: Vec<Node> = vec![];

                            while match token_iter.peek() {
                                      Some(&Token::ParenClosing) |
                                      None => false,
                                      _ => true,
                                  } {
                                match walk(token_iter.next().unwrap(), token_iter) {
                                    Ok(nodes) => params.push(nodes),
                                    Err(value) => return Err(value),
                                }
                            }

                            // skip Token::ParenClosing
                            token_iter.next().unwrap();

                            Ok(Node::CallExpression {
                                   name: name,
                                   params: params,
                               })
                        }
                        _ => {
                            return Err(format!("{:?} isn't followed by a {:?}.",
                                               Token::ParenOpening,
                                               Token::Name("example".to_string())))
                        }
                    }
                } else {
                    return Err(format!("{:?} isn't followed by a node.", Token::ParenOpening));
                }
            }
            _ => {
                return Err(format!("I don't know what this token is or this is a token which \
                                    should be covered elsewhere: {:?}",
                                   token))
            }
        }
    }

    let mut body: Vec<Node> = vec![];

    let mut token_iter = tokens.into_iter().peekable();
    while let Some(token) = token_iter.next() {
        match walk(token, &mut token_iter) {
            Ok(nodes) => body.push(nodes),
            Err(value) => return Err(value),
        }
    }

    Ok(Node::Program { body: body })
}

pub struct Visitor {
    pub enter: Option<Box<Fn(&Node, Option<&Node>, &mut Vec<TransformedNode>)>>,
    pub exit: Option<Box<Fn(&Node, Option<&Node>, &mut Vec<TransformedNode>)>>,
}

pub fn traverser(ast: Node,
                 mut context: &mut Vec<TransformedNode>,
                 visitors: &HashMap<NodeType, Visitor>) {
    fn traverse_nodes(nodes: &Vec<Node>,
                      parent: Option<&Node>,
                      mut context: &mut Vec<TransformedNode>,
                      visitors: &HashMap<NodeType, Visitor>) {
        for node in nodes {
            traverse_node(&node, parent, &mut context, visitors);
        }
    };

    fn traverse_node(node: &Node,
                     parent: Option<&Node>,
                     mut context: &mut Vec<TransformedNode>,
                     visitors: &HashMap<NodeType, Visitor>) {
        let visitor = visitors.get(&node.get_type());

        if visitor.is_some() {
            if let Some(ref enter) = visitor.unwrap().enter {
                enter(&node, parent, context);
            }
        }

        match *node {
            Node::Program { ref body } => traverse_nodes(body, Some(node), &mut context, visitors),
            Node::CallExpression { ref params, .. } => {
                traverse_nodes(params, Some(node), &mut context, visitors)
            }
            _ => {}
            //_ => println!("We can't have an unknown node here!"),
        }

        if visitor.is_some() {
            if let Some(ref exit) = visitor.unwrap().exit {
                exit(&node, parent, context);
            }
        }
    };

    traverse_node(&ast, None, &mut context, visitors);
}

#[derive(Debug,PartialEq,Clone)]
pub enum TransformedNode {
    Program { body: Vec<TransformedNode> },
    ExpressionStatement { expression: Box<TransformedNode> },
    CallExpression {
        callee: Box<TransformedNode>,
        arguments: Vec<TransformedNode>,
    },
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
}

pub fn transformer(ast: Node) -> TransformedNode {
    let mut visitors = HashMap::new();

    visitors.insert(
        NodeType::NumberLiteral,
        Visitor {
            enter: Some(Box::new(|
                node: &Node,
                _parent: Option<&Node>,
                context: &mut Vec<TransformedNode>
            | {
                if let Node::NumberLiteral(ref value) = *node {
                    context.push(TransformedNode::NumberLiteral(value.to_string()));
                }
            })),
            exit: None,
        }
    );

    visitors.insert(
        NodeType::StringLiteral,
        Visitor {
            enter: Some(Box::new(|
                node: &Node,
                _parent: Option<&Node>,
                context: &mut Vec<TransformedNode>
            | {
                if let Node::StringLiteral(ref value) = *node {
                    context.push(TransformedNode::StringLiteral(value.to_string()));
                }
            })),
            exit: None,
        }
    );

    visitors.insert(
        NodeType::CallExpression,
        Visitor {
            enter: Some(Box::new(|
                node: &Node,
                parent: Option<&Node>,
                context: &mut Vec<TransformedNode>
            | {
                if let Node::CallExpression { ref name, .. } = *node {
                    let mut arguments: Vec<TransformedNode> = vec![];

                    // context = &mut arguments;

                    let call_expression = TransformedNode::CallExpression {
                        callee: Box::new(TransformedNode::Identifier(name.to_string())),
                        arguments: arguments,
                    };

                    let expression = match parent {
                        Some(&Node::CallExpression { .. }) => call_expression,
                        _ => TransformedNode::ExpressionStatement { expression: Box::new(call_expression) },
                    };

                    context.push(expression);
                }
            })),
            exit: None,
        }
    );

    let mut body: Vec<TransformedNode> = vec![];

    traverser(ast, &mut body, &visitors);

    TransformedNode::Program { body: body }
}
