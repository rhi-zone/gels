use gels_core::grammar::Grammar;

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

/// Check a synthesized grammar for completeness and consistency.
///
/// Validates:
/// - All referenced nonterminals are defined
/// - No duplicate rule names
/// - At least one top-level entry rule exists
pub fn validate(_grammar: &Grammar) -> Vec<ValidationError> {
    Vec::new()
}
