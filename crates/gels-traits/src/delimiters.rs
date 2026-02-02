use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;
use gels_core::traits::{Confidence, DetectionResult, SyntaxTrait, TraitId};

pub struct BraceDelimited;
pub struct ParenDelimited;
pub struct BracketDelimited;
pub struct AngleBracketDelimited;

impl SyntaxTrait for BraceDelimited {
    fn id(&self) -> TraitId {
        TraitId("brace_delimited")
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

impl SyntaxTrait for ParenDelimited {
    fn id(&self) -> TraitId {
        TraitId("paren_delimited")
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
            nonterminals_provided: vec!["parenthesized_expression".into()],
            nonterminals_required: vec!["_expression".into()],
        }
    }
}

impl SyntaxTrait for BracketDelimited {
    fn id(&self) -> TraitId {
        TraitId("bracket_delimited")
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
            nonterminals_provided: vec!["subscript_expression".into()],
            nonterminals_required: vec!["_expression".into()],
        }
    }
}

impl SyntaxTrait for AngleBracketDelimited {
    fn id(&self) -> TraitId {
        TraitId("angle_bracket_delimited")
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
            nonterminals_provided: vec!["type_arguments".into()],
            nonterminals_required: vec!["_type".into()],
        }
    }
}
