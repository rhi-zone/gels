# Getting Started

## Installation

Gels is part of the rhi ecosystem. Add it to your workspace:

```toml
[dependencies]
gels = { git = "https://github.com/rhi-zone/gels" }
```

Or use the CLI directly:

```bash
cargo install --git https://github.com/rhi-zone/gels gels
```

## Development setup

```bash
git clone https://github.com/rhi-zone/gels
cd gels
nix develop    # Enter dev shell (provides Rust toolchain + tools)
cargo build
cargo test
```

## Running the CLI

Point `gels` at a directory containing source files:

```bash
# Show which syntactic traits were detected and how confidently
gels analyze ./src

# Output:
# brace_delimited: 0.92
# semicolon_terminated: 0.88
# infix_operators: 0.74
# type_annotation_colon: 0.61
# ...

# Attempt to identify the language
gels identify ./src
# Identified: rust

# Synthesize a tree-sitter grammar for an unknown language
gels emit ./my-dsl -o grammar.js
```

The `analyze` command prints all traits with confidence above 0. The `identify` command
only reports a match if the best profile scores above 0.5.

## Using the library

```rust
use gels;
use gels_synth;

fn main() {
    let source = std::fs::read_to_string("main.rs").unwrap();

    // Run trait detection
    let results = gels::analyze(&source);
    for r in &results {
        if r.confidence.0 > 0.5 {
            println!("{}: {:.2}", r.trait_id.0, r.confidence.0);
        }
    }

    // Identify language
    match gels::identify(&source) {
        Some(name) => println!("Language: {name}"),
        None => {
            // Unknown — synthesize a grammar
            let grammar = gels::synthesize(&source, "unknown");
            let js = gels_synth::output::emit(&grammar);
            println!("{js}");
        }
    }
}
```
