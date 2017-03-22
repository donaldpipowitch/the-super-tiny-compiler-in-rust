extern crate the_super_tiny_compiler;

use the_super_tiny_compiler::*;
use std::collections::HashMap;

#[test]
fn tokenizer_works() {
    // let input = "(add 2 (subtract 4 2))";
    let input = "(add 22 \"ff\" (subtract 4 2))";

    let tokens = vec![Token::ParenOpening,
                      Token::Name("add".to_string()),
                      Token::Number("22".to_string()),
                      Token::String("ff".to_string()),
                      Token::ParenOpening,
                      Token::Name("subtract".to_string()),
                      Token::Number("4".to_string()),
                      Token::Number("2".to_string()),
                      Token::ParenClosing,
                      Token::ParenClosing];

    let ast = Node::Programm {
        body: vec![Node::CallExpression {
                       name: "add".to_string(),
                       params: vec![Node::NumberLiteral("22".to_string()),
                                    Node::StringLiteral("ff".to_string()),
                                    Node::CallExpression {
                                        name: "subtract".to_string(),
                                        params: vec![Node::NumberLiteral("4".to_string()),
                                                     Node::NumberLiteral("2".to_string())],
                                    }],
                   }],
    };

    assert_eq!(Ok(tokens.clone()), tokenizer(input));
    assert_eq!(Ok(ast), parser(tokens.clone()));

    // test traverser
    let mut visitors = HashMap::new();
    visitors.insert(NodeType::Programm,
                    Visitor {
                        enter: Some(Box::new(|node: &Node, parent: Option<&Node>| {
                            println!("test enter works!")
                        })),
                        exit: Some(Box::new(|node: &Node, parent: Option<&Node>| {
                            println!("test exit works!")
                        })), // exit: None,
                    });
    traverser(parser(tokens.clone()).unwrap(), visitors);
}
