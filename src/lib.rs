#[derive(Debug,PartialEq)]
pub enum Token {
    Number(String),
    // String(String),
    // Name(String),
    SomethingElse,
    ParenOpening,
    ParenClosing,
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();
    while let Some(mut c) = char_iter.next() {
        match c {
            c if c.is_whitespace() => (),
            '(' => tokens.push(Token::ParenOpening),
            ')' => tokens.push(Token::ParenClosing),
            '0'...'9' => {
                let mut value = String::new();
                value.push(c);

                while match char_iter.peek() {
                    Some(&'0'...'9') => true,
                    _ => false,
                } {
                    value.push(c);
                    match char_iter.next() {
                        Some(v) => c = v,
                        None => break,
                    }
                }
                tokens.push(Token::Number(value));
            }
            _ => tokens.push(Token::SomethingElse),
        }
    }

    tokens
}
