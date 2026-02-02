use gels_core::emit::emit_grammar_js;
use gels_core::grammar::Grammar;

/// Emit a tree-sitter grammar.js file from a Grammar.
pub fn emit(grammar: &Grammar) -> String {
    emit_grammar_js(grammar)
}
