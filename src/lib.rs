#[derive(Debug,PartialEq)]
pub enum Token {
    // Number(String),
    // String(String),
    // Name(String),
    ParenOpening,
    ParenClosing,
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    for char in input.chars() {
        match char {
            '(' => tokens.push(Token::ParenOpening),
            _ => tokens.push(Token::ParenClosing),
        }
    }

    tokens
}
