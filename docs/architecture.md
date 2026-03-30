# Architecture

## Crate structure

```
gels              # CLI binary + public library API
gels-core         # Token types, SyntaxTrait interface, grammar IR, language profiles
gels-traits       # Language-agnostic tokenizer, built-in trait detectors, known language registry
gels-synth        # Grammar fragment merging, validation, tree-sitter grammar.js emission
```

The crates form a directed dependency graph:

```
gels
 ├── gels-core
 ├── gels-traits  → gels-core
 └── gels-synth   → gels-core
```

## Pipeline

```
source files
     │
     ▼
  tokenize          (gels-traits::tokenize)
     │
     ▼
  detect traits     (gels-traits::register_all → SyntaxTrait::detect for each)
     │
     ├── identify   (gels-traits::registry::known_profiles → score matching)
     │
     └── synthesize (gels-synth::merge::merge_fragments → Grammar)
                              │
                              ▼
                        emit grammar.js  (gels-core::emit)
```

## Core types (gels-core)

### Token

The tokenizer produces a flat `Vec<Token>`. Each token has a `TokenKind`, a text slice,
and a byte-offset span.

```rust
pub enum TokenKind {
    Identifier, Keyword, Operator, Punctuation,
    StringLiteral, NumberLiteral, Comment,
    Whitespace, Newline, Indent, Dedent, Unknown,
}
```

### SyntaxTrait

The central interface. Each detector implements three methods:

```rust
pub trait SyntaxTrait: Send + Sync {
    fn id(&self) -> TraitId;
    fn detect(&self, tokens: &[Token]) -> DetectionResult;
    fn grammar_fragment(&self) -> GrammarFragment;
}
```

`detect` returns a confidence in `[0, 1]` and a list of evidence records pointing to
token indices. `grammar_fragment` returns the grammar rules that should be included
when this trait is present.

### Grammar IR

The grammar IR mirrors tree-sitter's `grammar.js` structure. `GrammarFragment` is a
partial grammar produced by a single trait detector. `Grammar` is the complete assembled
result.

```rust
pub enum Rule {
    String(String), Pattern(String), Symbol(String),
    Seq(Vec<Rule>), Choice(Vec<Rule>),
    Repeat(Box<Rule>), Repeat1(Box<Rule>), Optional(Box<Rule>),
    Prec(Precedence, Box<Rule>), Token(Box<Rule>), Field(String, Box<Rule>),
}
```

### LanguageProfile

A named language is described by the traits it exhibits and their expected confidences,
paired with its known grammar:

```rust
pub struct LanguageProfile {
    pub name: String,
    pub traits: Vec<(TraitId, Confidence)>,
    pub grammar: Grammar,
}
```

## Trait detectors (gels-traits)

Built-in detectors cover the most common syntactic dimensions:

| Module | Traits |
|--------|--------|
| `delimiters` | `brace_delimited`, `paren_delimited`, `bracket_delimited`, `angle_bracket_delimited` |
| `terminators` | `semicolon_terminated`, `newline_terminated` |
| `blocks` | `keyword_block_delimited`, `indentation_scoped` |
| `operators` | `infix_operators`, `prefix_operators` |
| `bindings` | `assignment_equals`, `assignment_arrow`, `type_annotation_colon`, `type_annotation_prefix`, `ml_let_binding` |
| `patterns` | `pattern_matching` |
| `strings` | `single_line_comment`, `multi_line_comment` |

All detectors are registered by `register_all()` and run against the same token stream.

## Language identification

`gels::identify` scores each known profile against the detection results:

```
score = Σ (detected_confidence × expected_confidence)  for each trait in the profile
```

The best-scoring profile is returned if its score exceeds 0.5. The threshold is
intentionally conservative — false positives are worse than unknowns.

Known profiles are returned by `gels_traits::registry::known_profiles()`. The registry
is currently empty; profiles will be added as the detection system matures.

## Grammar synthesis

When identification fails, `gels::synthesize` merges the grammar fragments of all
detected traits above a confidence threshold (currently 0.3) into a single `Grammar`.

The merge algorithm (`gels_synth::merge`):

1. Collect fragments from traits above the threshold
2. Resolve shared nonterminals (`_expression`, `_statement`, `_declaration`, `_block`)
3. Apply confidence-weighted conflict resolution when two traits define the same nonterminal
4. Return the merged grammar

The resulting `Grammar` is emitted as a valid `grammar.js` file by
`gels_synth::output::emit` (which calls `gels_core::emit::emit_grammar_js`).

## Current state

The infrastructure — tokenizer, trait interfaces, grammar IR, CLI, synthesis pipeline —
is fully in place. Most trait `detect` implementations currently return confidence 0;
the actual token-level detection logic is the main area of active development. The merge
algorithm is similarly stubbed — it returns an empty grammar — pending the design of
nonterminal resolution.
