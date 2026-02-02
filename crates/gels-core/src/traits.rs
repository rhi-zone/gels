use crate::grammar::GrammarFragment;
use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Confidence(pub f64);

impl Confidence {
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }
}

#[derive(Debug, Clone)]
pub struct Evidence {
    pub description: String,
    pub token_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub trait_id: TraitId,
    pub confidence: Confidence,
    pub evidence: Vec<Evidence>,
}

pub trait SyntaxTrait: Send + Sync {
    fn id(&self) -> TraitId;
    fn detect(&self, tokens: &[Token]) -> DetectionResult;
    fn grammar_fragment(&self) -> GrammarFragment;
}
