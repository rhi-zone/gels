use gels_core::token::{Span, Token, TokenKind};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some(&(i, ch)) = chars.peek() {
        let token = match ch {
            '\n' => {
                chars.next();
                Token {
                    kind: TokenKind::Newline,
                    text: "\n".into(),
                    span: Span {
                        start: i,
                        end: i + 1,
                    },
                }
            }
            c if c.is_whitespace() => {
                let start = i;
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_whitespace() && c != '\n' {
                        chars.next();
                    } else {
                        break;
                    }
                }
                let end = chars.peek().map_or(input.len(), |&(j, _)| j);
                Token {
                    kind: TokenKind::Whitespace,
                    text: input[start..end].into(),
                    span: Span { start, end },
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        chars.next();
                    } else {
                        break;
                    }
                }
                let end = chars.peek().map_or(input.len(), |&(j, _)| j);
                Token {
                    kind: TokenKind::Identifier,
                    text: input[start..end].into(),
                    span: Span { start, end },
                }
            }
            c if c.is_ascii_digit() => {
                let start = i;
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' || c == '_' {
                        chars.next();
                    } else {
                        break;
                    }
                }
                let end = chars.peek().map_or(input.len(), |&(j, _)| j);
                Token {
                    kind: TokenKind::NumberLiteral,
                    text: input[start..end].into(),
                    span: Span { start, end },
                }
            }
            '"' | '\'' => {
                let quote = ch;
                let start = i;
                chars.next();
                while let Some(&(_, c)) = chars.peek() {
                    chars.next();
                    if c == quote {
                        break;
                    }
                    if c == '\\' {
                        chars.next();
                    }
                }
                let end = chars.peek().map_or(input.len(), |&(j, _)| j);
                Token {
                    kind: TokenKind::StringLiteral,
                    text: input[start..end].into(),
                    span: Span { start, end },
                }
            }
            '{' | '}' | '(' | ')' | '[' | ']' | ',' | ';' | ':' => {
                chars.next();
                Token {
                    kind: TokenKind::Punctuation,
                    text: ch.to_string(),
                    span: Span {
                        start: i,
                        end: i + 1,
                    },
                }
            }
            _ => {
                chars.next();
                Token {
                    kind: TokenKind::Operator,
                    text: ch.to_string(),
                    span: Span {
                        start: i,
                        end: i + ch.len_utf8(),
                    },
                }
            }
        };
        tokens.push(token);
    }

    tokens
}
