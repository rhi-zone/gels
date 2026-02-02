use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;

/// Extract likely keywords from token frequency and positional analysis.
/// Returns (keyword, count) pairs sorted by confidence.
pub fn extract_keywords(_tokens: &[Token]) -> Vec<(String, usize)> {
    // Keyword extraction will analyze identifier frequency, position patterns
    // (statement-initial, always-lowercase), and co-occurrence with structural tokens.
    Vec::new()
}

/// Generate grammar fragment for a set of detected keywords.
pub fn keywords_fragment(_keywords: &[(String, usize)]) -> GrammarFragment {
    GrammarFragment {
        rules: Vec::new(),
        precedences: Vec::new(),
        nonterminals_provided: Vec::new(),
        nonterminals_required: Vec::new(),
    }
}
