use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rule {
    /// Match a literal string
    String(String),
    /// Match a regex pattern
    Pattern(String),
    /// Reference another named rule
    Symbol(String),
    /// Sequence of rules
    Seq(Vec<Rule>),
    /// Choice between rules
    Choice(Vec<Rule>),
    /// Zero or more repetitions
    Repeat(Box<Rule>),
    /// One or more repetitions
    Repeat1(Box<Rule>),
    /// Optional rule
    Optional(Box<Rule>),
    /// Precedence wrapper
    Prec(Precedence, Box<Rule>),
    /// Token (force into a single token)
    Token(Box<Rule>),
    /// Field name binding
    Field(String, Box<Rule>),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Precedence {
    None,
    Left(i32),
    Right(i32),
    Dynamic(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecedenceLevel {
    pub level: i32,
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarFragment {
    pub rules: Vec<(String, Rule)>,
    pub precedences: Vec<PrecedenceLevel>,
    pub nonterminals_provided: Vec<String>,
    pub nonterminals_required: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Grammar {
    pub name: String,
    pub rules: Vec<(String, Rule)>,
    pub precedences: Vec<PrecedenceLevel>,
    pub extras: Vec<Rule>,
    pub externals: Vec<Rule>,
    pub conflicts: Vec<Vec<String>>,
}
