use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;
use gels_core::traits::{Confidence, DetectionResult, SyntaxTrait, TraitId};

pub struct PatternMatching;

impl SyntaxTrait for PatternMatching {
    fn id(&self) -> TraitId {
        TraitId("pattern_matching")
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
            nonterminals_provided: vec!["match_expression".into()],
            nonterminals_required: vec!["_expression".into(), "_pattern".into()],
        }
    }
}
