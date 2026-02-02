use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;
use gels_core::traits::{Confidence, DetectionResult, SyntaxTrait, TraitId};

pub struct SingleLineComment;
pub struct MultiLineComment;

impl SyntaxTrait for SingleLineComment {
    fn id(&self) -> TraitId {
        TraitId("single_line_comment")
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
            nonterminals_provided: vec!["comment".into()],
            nonterminals_required: Vec::new(),
        }
    }
}

impl SyntaxTrait for MultiLineComment {
    fn id(&self) -> TraitId {
        TraitId("multi_line_comment")
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
            nonterminals_provided: vec!["comment".into()],
            nonterminals_required: Vec::new(),
        }
    }
}
