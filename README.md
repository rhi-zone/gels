# gels

Trait-based grammar inference engine: detect syntactic traits from source files, identify
the language, or synthesize a tree-sitter grammar for unknown ones.

## What it does

Given a directory of source files, gels runs them through a pipeline:

1. **Tokenize** — a language-agnostic tokenizer produces a flat token stream.
2. **Detect traits** — independent `SyntaxTrait` detectors each examine the token stream
   for one syntactic property (brace-delimited blocks, semicolon terminators, ML-style let
   bindings, pattern matching, and more) and return a confidence score.
3. **Identify or synthesize** — detected traits are score-matched against known language
   profiles. A match above threshold identifies the language; otherwise, the trait
   fragments are merged into a synthesized tree-sitter `grammar.js`.

This gives a path from "some source files in an unfamiliar language" to "a parseable
grammar" without prior knowledge of that language.

## Key features

- **Trait detection** — each syntactic property is its own detector with an independent
  confidence score.
- **Language identification** — score-weighted matching of detected trait profiles against
  known language fingerprints.
- **Grammar synthesis** — merges detected trait fragments into a usable tree-sitter grammar
  for unrecognized languages.
- **Extensible** — implement `SyntaxTrait` to add new detectors; register language profiles
  to extend identification coverage.

## Workspace

- `gels-core` — grammar, profile, and trait types shared across the workspace.
- `gels-traits` — the trait detectors and the tokenizer.
- `gels-synth` — merges detected traits into a grammar and emits tree-sitter output.
- `gels` — the CLI and library entry point (`analyze`, `identify`, `synthesize`).

Full documentation: [docs.rhi.zone/gels](https://docs.rhi.zone/gels/)

## License

Licensed under MIT OR Apache-2.0.
