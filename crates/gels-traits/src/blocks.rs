use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;
use gels_core::traits::{Confidence, DetectionResult, SyntaxTrait, TraitId};

pub struct KeywordBlockDelimited;
pub struct IndentationScoped;

impl SyntaxTrait for KeywordBlockDelimited {
    fn id(&self) -> TraitId {
        TraitId("keyword_block_delimited")
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
            nonterminals_provided: vec!["_block".into()],
            nonterminals_required: vec!["_statement".into()],
        }
    }
}

impl SyntaxTrait for IndentationScoped {
    fn id(&self) -> TraitId {
        TraitId("indentation_scoped")
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
            nonterminals_provided: vec!["_block".into()],
            nonterminals_required: vec!["_statement".into()],
        }
    }
}
