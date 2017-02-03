extern crate the_super_tiny_compiler;

use the_super_tiny_compiler::*;

#[test]
fn tokenizer_works() {
    let input = "(add 2 (subtract 4 2))";
    // let input = "(add 22 (subtract 4 2))";

    let tokens = vec![Token::ParenOpening,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::Number("2".to_string()),
                      //   Token::Number("22".to_string()),
                      Token::ParenOpening,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::SomethingElse,
                      Token::Number("4".to_string()),
                      Token::Number("2".to_string()),
                      Token::ParenClosing,
                      Token::ParenClosing];

    assert_eq!(tokens, tokenizer(input));
}
