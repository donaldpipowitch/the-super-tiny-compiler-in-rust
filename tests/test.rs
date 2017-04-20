extern crate the_super_tiny_compiler;

use the_super_tiny_compiler::*;

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

    let ast = Node::Program {
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

    let new_ast = TransformedNode::Program {
        body: vec![
            TransformedNode::ExpressionStatement {
                expression: Box::new(TransformedNode::CallExpression {
                    callee: Box::new(TransformedNode::Identifier("add".to_string())),
                    arguments: vec![
                        TransformedNode::NumberLiteral("22".to_string()),
                        TransformedNode::StringLiteral("ff".to_string()),
                        TransformedNode::CallExpression {
                            callee: Box::new(TransformedNode::Identifier("subtract".to_string())),
                            arguments: vec![
                                TransformedNode::NumberLiteral("4".to_string()),
                                TransformedNode::NumberLiteral("2".to_string())
                            ]
                        }
                    ]
                })

            }
        ],
    };

    assert_eq!(tokenizer(input), Ok(tokens.clone()), "Tokenizer should turn `input` into `tokens`");
    assert_eq!(parser(tokens.clone()), Ok(ast.clone()), "Parser should turn `tokens` into `ast`");
    assert_eq!(transformer(ast.clone()), new_ast, "Transformer should turn `ast` into a `new_ast`");
}
