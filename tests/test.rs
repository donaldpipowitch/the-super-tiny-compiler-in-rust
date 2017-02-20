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

    let ast = Ast::Programm {
        body: vec![Ast::CallExpression {
                       name: "add".to_string(),
                       params: vec![Ast::NumberLiteral("22".to_string()),
                                    Ast::StringLiteral("ff".to_string()),
                                    Ast::CallExpression {
                                        name: "subtract".to_string(),
                                        params: vec![Ast::NumberLiteral("4".to_string()),
                                                     Ast::NumberLiteral("2".to_string())],
                                    }],
                   }],
    };

    assert_eq!(Ok(tokens), tokenizer(input));

    let tokens2 = vec![Token::ParenOpening,
                       Token::Name("add".to_string()),
                       Token::Number("22".to_string()),
                       Token::String("ff".to_string()),
                       Token::ParenOpening,
                       Token::Name("subtract".to_string()),
                       Token::Number("4".to_string()),
                       Token::Number("2".to_string()),
                       Token::ParenClosing,
                       Token::ParenClosing];
    assert_eq!(Ok(ast), parser(tokens2));
}
