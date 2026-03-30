---
layout: home

hero:
  name: Gels
  text: Grammar Inference Engine
  tagline: Detect syntactic traits from source files. Identify languages or synthesize tree-sitter grammars.
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/rhi-zone/gels

features:
  - title: Trait Detection
    details: Each syntactic property — brace-delimited blocks, semicolon terminators, ML-style let bindings — is its own detector with an independent confidence score.
  - title: Language Identification
    details: Match detected trait profiles against known language fingerprints. Score-weighted matching returns the best candidate or reports an unknown language.
  - title: Grammar Synthesis
    details: For unrecognized languages, merge detected trait fragments into a valid tree-sitter grammar.js that can be used immediately.
  - title: Extensible
    details: Implement the SyntaxTrait interface to add new detectors. Register language profiles to extend identification coverage.
---
