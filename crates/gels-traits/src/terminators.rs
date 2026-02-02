use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;
use gels_core::traits::{Confidence, DetectionResult, SyntaxTrait, TraitId};

pub struct SemicolonTerminated;
pub struct NewlineTerminated;

impl SyntaxTrait for SemicolonTerminated {
    fn id(&self) -> TraitId {
        TraitId("semicolon_terminated")
    }

    fn detect(&self, _tokens: &[Token]) -> DetectionResult {
        DetectionResult {
            trait_id: self.id(),
            confidence: Confidence(0.0),
            evidence: Vec::new(),
        }
    }

    fn grammar_fragment(&self) -> GrammarFragment {
        GrammarFragment {
            rules: Vec::new(),
            precedences: Vec::new(),
            nonterminals_provided: vec!["_statement_terminator".into()],
            nonterminals_required: Vec::new(),
        }
    }
}

impl SyntaxTrait for NewlineTerminated {
    fn id(&self) -> TraitId {
        TraitId("newline_terminated")
    }

    fn detect(&self, _tokens: &[Token]) -> DetectionResult {
        DetectionResult {
            trait_id: self.id(),
            confidence: Confidence(0.0),
            evidence: Vec::new(),
        }
    }

    fn grammar_fragment(&self) -> GrammarFragment {
        GrammarFragment {
            rules: Vec::new(),
            precedences: Vec::new(),
            nonterminals_provided: vec!["_statement_terminator".into()],
            nonterminals_required: Vec::new(),
        }
    }
}
