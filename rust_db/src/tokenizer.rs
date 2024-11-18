// src/tokenizer.rs
use crate::errors::DatabaseError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    Operator(char),
    Punctuation(char),
}

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, DatabaseError> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        
        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\n' => { chars.next(); }
                'a'..='z' | 'A'..='Z' => {
                    let word = Tokenizer::collect_word(&mut chars);
                    tokens.push(Tokenizer::classify_token(&word));
                }
                '0'..='9' | '\'' => {
                    let literal = Tokenizer::collect_literal(&mut chars);
                    tokens.push(Token::Literal(literal));
                }
                '=' | '<' | '>' | '+' | '-' | '*' | '/' => {
                    tokens.push(Token::Operator(chars.next().unwrap()));
                }
                '(' | ')' | ',' => {
                    tokens.push(Token::Punctuation(chars.next().unwrap()));
                }
                _ => return Err(DatabaseError::ParseError(format!("Invalid token: {}", ch)))
            }
        }
        
        Ok(tokens)
    }
    
    fn collect_word(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut word = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() {
                word.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        word
    }
    
    fn collect_literal(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut literal = String::new();
        
        // Handle numeric literals
        if let Some(&ch) = chars.peek() {
            if ch.is_numeric() {
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() || c == '.' {
                        literal.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                return literal;
            }
        }
        
        // Handle string literals
        if let Some(quote) = chars.next() {
            if quote == '\'' {
                while let Some(&ch) = chars.peek() {
                    if ch == '\'' {
                        chars.next(); // consume closing quote
                        break;
                    }
                    literal.push(chars.next().unwrap());
                }
            }
        }
        
        literal
    }
    
    fn classify_token(word: &str) -> Token {
        let keywords = ["CREATE", "TABLE", "INSERT", "INTO", "VALUES", "SELECT", "FROM", "DELETE", "WHERE"];
        if keywords.contains(&word.to_uppercase().as_str()) {
            Token::Keyword(word.to_string())
        } else {
            Token::Identifier(word.to_string())
        }
    }
}