extern crate the_super_tiny_compiler;

use the_super_tiny_compiler::*;

#[test]
fn tokenizer_works() {
    let input = "(add 2 (subtract 4 2))";

    let tokens = vec![Token::ParenOpening,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenOpening,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing,
                      Token::ParenClosing];

    let tokenizedInput = tokenizer(input);

    assert_eq!(tokens.len(), tokenizedInput.len());
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token, tokenizedInput[i]);
    }
}
