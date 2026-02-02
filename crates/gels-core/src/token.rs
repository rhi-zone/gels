use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenKind {
    Identifier,
    Keyword,
    Operator,
    Punctuation,
    StringLiteral,
    NumberLiteral,
    Comment,
    Whitespace,
    Newline,
    Indent,
    Dedent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span,
}
