use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;
use gels_core::traits::{Confidence, DetectionResult, SyntaxTrait, TraitId};

pub struct AssignmentEquals;
pub struct AssignmentArrow;
pub struct TypeAnnotationColon;
pub struct TypeAnnotationPrefix;
pub struct MLLetBinding;

impl SyntaxTrait for AssignmentEquals {
    fn id(&self) -> TraitId {
        TraitId("assignment_equals")
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
            nonterminals_provided: vec!["assignment_expression".into()],
            nonterminals_required: vec!["_expression".into()],
        }
    }
}

impl SyntaxTrait for AssignmentArrow {
    fn id(&self) -> TraitId {
        TraitId("assignment_arrow")
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
            nonterminals_provided: vec!["assignment_expression".into()],
            nonterminals_required: vec!["_expression".into()],
        }
    }
}

impl SyntaxTrait for TypeAnnotationColon {
    fn id(&self) -> TraitId {
        TraitId("type_annotation_colon")
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
            nonterminals_provided: vec!["type_annotation".into()],
            nonterminals_required: vec!["_type".into()],
        }
    }
}

impl SyntaxTrait for TypeAnnotationPrefix {
    fn id(&self) -> TraitId {
        TraitId("type_annotation_prefix")
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
            nonterminals_provided: vec!["type_annotation".into()],
            nonterminals_required: vec!["_type".into()],
        }
    }
}

impl SyntaxTrait for MLLetBinding {
    fn id(&self) -> TraitId {
        TraitId("ml_let_binding")
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
            nonterminals_provided: vec!["let_binding".into()],
            nonterminals_required: vec!["_expression".into(), "_pattern".into()],
        }
    }
}
