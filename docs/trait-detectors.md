# Trait Detectors

A trait detector is any type that implements `SyntaxTrait`. Detectors are independent —
each examines the full token stream and returns its own confidence score. There is no
ordering dependency between detectors.

## The SyntaxTrait interface

```rust
pub trait SyntaxTrait: Send + Sync {
    fn id(&self) -> TraitId;
    fn detect(&self, tokens: &[Token]) -> DetectionResult;
    fn grammar_fragment(&self) -> GrammarFragment;
}
```

- **`id`** — A static string identifier, e.g. `"brace_delimited"`. Used as the key in detection results and language profiles.
- **`detect`** — Examines the token stream and returns a `DetectionResult` containing a confidence in `[0, 1]` and optional evidence records.
- **`grammar_fragment`** — Returns the grammar rules that should be contributed to the synthesized grammar when this trait is detected. Declares which nonterminals it provides and which it requires.

## Detection result

```rust
pub struct DetectionResult {
    pub trait_id: TraitId,
    pub confidence: Confidence,   // f64 in [0, 1]
    pub evidence: Vec<Evidence>,
}

pub struct Evidence {
    pub description: String,
    pub token_index: Option<usize>,  // Points into the token stream
}
```

Evidence records are informational — they let callers understand why a trait was
detected, and can be used to highlight relevant tokens in a UI.

## Grammar fragments

Each detector declares what it contributes structurally:

```rust
pub struct GrammarFragment {
    pub rules: Vec<(String, Rule)>,          // Named grammar rules
    pub precedences: Vec<PrecedenceLevel>,   // Precedence declarations
    pub nonterminals_provided: Vec<String>,  // What this fragment defines
    pub nonterminals_required: Vec<String>,  // What it depends on
}
```

The `nonterminals_provided` / `nonterminals_required` declarations are used during
synthesis to wire fragments together and detect conflicts.

## Built-in detectors

### Delimiters

| ID | Struct | Provides |
|----|--------|----------|
| `brace_delimited` | `BraceDelimited` | `_block` |
| `paren_delimited` | `ParenDelimited` | `parenthesized_expression` |
| `bracket_delimited` | `BracketDelimited` | `subscript_expression` |
| `angle_bracket_delimited` | `AngleBracketDelimited` | `type_arguments` |

### Terminators

| ID | Struct | Provides |
|----|--------|----------|
| `semicolon_terminated` | `SemicolonTerminated` | `_statement` |
| `newline_terminated` | `NewlineTerminated` | `_statement` |

### Blocks

| ID | Struct | Provides |
|----|--------|----------|
| `keyword_block_delimited` | `KeywordBlockDelimited` | `_block` |
| `indentation_scoped` | `IndentationScoped` | `_block` |

### Operators

| ID | Struct | Provides |
|----|--------|----------|
| `infix_operators` | `InfixOperators` | `binary_expression` |
| `prefix_operators` | `PrefixOperators` | `unary_expression` |

### Bindings

| ID | Struct | Provides |
|----|--------|----------|
| `assignment_equals` | `AssignmentEquals` | `assignment` |
| `assignment_arrow` | `AssignmentArrow` | `assignment` |
| `type_annotation_colon` | `TypeAnnotationColon` | `type_annotation` |
| `type_annotation_prefix` | `TypeAnnotationPrefix` | `type_annotation` |
| `ml_let_binding` | `MLLetBinding` | `let_binding` |

### Patterns

| ID | Struct | Provides |
|----|--------|----------|
| `pattern_matching` | `PatternMatching` | `match_expression` |

### Strings / Comments

| ID | Struct | Provides |
|----|--------|----------|
| `single_line_comment` | `SingleLineComment` | `comment` |
| `multi_line_comment` | `MultiLineComment` | `comment` |

## Writing a custom detector

Implement `SyntaxTrait` and register your detector:

```rust
use gels_core::grammar::GrammarFragment;
use gels_core::token::Token;
use gels_core::traits::{Confidence, DetectionResult, Evidence, SyntaxTrait, TraitId};

pub struct MyTrait;

impl SyntaxTrait for MyTrait {
    fn id(&self) -> TraitId {
        TraitId("my_trait")
    }

    fn detect(&self, tokens: &[Token]) -> DetectionResult {
        let mut hits = 0usize;
        let mut evidence = Vec::new();

        for (i, token) in tokens.iter().enumerate() {
            if /* matches some pattern */ false {
                hits += 1;
                evidence.push(Evidence {
                    description: "found my pattern".into(),
                    token_index: Some(i),
                });
            }
        }

        let confidence = if tokens.is_empty() {
            0.0
        } else {
            (hits as f64 / tokens.len() as f64).clamp(0.0, 1.0)
        };

        DetectionResult {
            trait_id: self.id(),
            confidence: Confidence::new(confidence),
            evidence,
        }
    }

    fn grammar_fragment(&self) -> GrammarFragment {
        GrammarFragment {
            rules: vec![], // add Rule entries here
            precedences: vec![],
            nonterminals_provided: vec!["my_nonterminal".into()],
            nonterminals_required: vec!["_expression".into()],
        }
    }
}
```

Pass an instance to `gels_traits::register_all`'s return value, or build your own
`Vec<Box<dyn SyntaxTrait>>` and call the detection pipeline manually via
`gels_core::traits` and `gels_synth`.
