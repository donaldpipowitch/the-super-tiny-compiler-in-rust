#[derive(Debug,PartialEq)]
pub enum Token {
    Number(String),
    String(String),
    Name(String),
    SomethingElse,
    ParenOpening,
    ParenClosing,
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();
    while let Some(c) = char_iter.next() {
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
                    if let Some(c) = char_iter.next() {
                        value.push(c);
                    }
                }
                tokens.push(Token::Number(value));
            }
            'a'...'z' => {
                let mut value = String::new();
                value.push(c);

                while match char_iter.peek() {
                    Some(&'a'...'z') => true,
                    _ => false,
                } {
                    if let Some(c) = char_iter.next() {
                        value.push(c);
                    }
                }
                tokens.push(Token::Name(value));
            }
            '"' => {
                let mut value = String::new();
                // skip opening "

                while match char_iter.peek() {
                    Some(&'"') | None => false,
                    _ => true,
                } {
                    if let Some(c) = char_iter.next() {
                        value.push(c);
                    }
                }
                tokens.push(Token::String(value));

                // skip closing ""
                if char_iter.peek() == Some(&'"') {
                    char_iter.next();
                }
            }
            // TODO: Use Option here...
            _ => tokens.push(Token::SomethingElse),
        }
    }

    tokens
}
