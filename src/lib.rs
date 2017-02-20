use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug,PartialEq)]
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
            _ => return Err(format!("I dont know what this character is: {}", c)),
        }
    }

    Ok(tokens)
}

#[derive(Debug,PartialEq)]
pub enum Ast {
    Programm { body: Vec<Ast> },
    CallExpression { name: String, params: Vec<Ast> },
    StringLiteral(String),
    NumberLiteral(String),
    SomethingElse,
}

pub fn parser(tokens: Vec<Token>) -> Result<Ast, String> {
    fn walk(token: &Token, token_iter: &mut Peekable<Iter<Token>>) -> Result<Ast, String> {
        match token {
            &Token::Number(ref value @ _) => Ok(Ast::NumberLiteral(value.to_string())),
            &Token::String(ref value @ _) => Ok(Ast::StringLiteral(value.to_string())),
            &Token::ParenOpening => {
                if let Some(token) = token_iter.next() {
                    match token {
                        &Token::Name(ref value @ _) => {
                            let name = value.to_string();
                            let mut params: Vec<Ast> = vec![];

                            while match token_iter.peek() {
                                Some(&&Token::ParenClosing) => {
                                    // skip closing )
                                    token_iter.next();
                                    false
                                }
                                None => false,
                                _ => true,
                            } {
                                if let Some(token) = token_iter.next() {
                                    match walk(token, token_iter) {
                                        Ok(nodes) => params.push(nodes),
                                        Err(value) => return Err(value),
                                    }
                                }
                            }

                            Ok(Ast::CallExpression {
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
            _ => return Err(format!("I dont know what this token is: {:?}", token)),
        }
    }

    let mut body: Vec<Ast> = vec![];

    let mut token_iter = tokens.iter().peekable();
    while let Some(token) = token_iter.next() {
        match walk(token, &mut token_iter) {
            Ok(nodes) => body.push(nodes),
            Err(value) => return Err(value),
        }
    }

    Ok(Ast::Programm { body: body })
}
