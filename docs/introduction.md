# Introduction

Gels is a trait-based grammar inference engine. Given a directory of source files, it
detects syntactic properties, identifies the language if known, or synthesizes a
tree-sitter grammar for unknown ones.

## The problem

Tree-sitter grammars are written by hand, language by language. When you encounter
source files in an unfamiliar language or a project-internal DSL, you have no grammar to
parse with. Gels provides a path from "some source files" to "a parseable grammar"
without needing prior knowledge of the language.

## How it works

Gels reads source files and runs them through a pipeline:

1. **Tokenize** — A language-agnostic tokenizer produces a flat token stream (identifiers, operators, punctuation, literals, whitespace, newlines).
2. **Detect traits** — Each `SyntaxTrait` detector examines the token stream and returns a confidence score between 0 and 1. Detectors are independent; they look for one syntactic property each.
3. **Identify or synthesize** — Detected traits are matched against known language profiles. If a profile matches above the threshold, the language is identified. Otherwise, trait fragments are merged into a synthesized tree-sitter grammar.

## CLI

```bash
# Detect traits and print confidence scores
gels analyze ./src

# Identify the language
gels identify ./src

# Synthesize a tree-sitter grammar
gels emit ./src -o grammar.js
```

## As a library

```rust
// Detect traits
let results = gels::analyze(&source);

// Identify language
if let Some(name) = gels::identify(&source) {
    println!("Identified: {name}");
}

// Synthesize grammar
let grammar = gels::synthesize(&source, "my_lang");
let js = gels_synth::output::emit(&grammar);
```
