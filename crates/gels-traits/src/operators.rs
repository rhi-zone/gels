use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;
use gels_core::traits::{Confidence, DetectionResult, SyntaxTrait, TraitId};

pub struct InfixOperators;
pub struct PrefixOperators;

impl SyntaxTrait for InfixOperators {
    fn id(&self) -> TraitId {
        TraitId("infix_operators")
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
            nonterminals_provided: vec!["binary_expression".into()],
            nonterminals_required: vec!["_expression".into()],
        }
    }
}

impl SyntaxTrait for PrefixOperators {
    fn id(&self) -> TraitId {
        TraitId("prefix_operators")
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
            nonterminals_provided: vec!["unary_expression".into()],
            nonterminals_required: vec!["_expression".into()],
        }
    }
}
