use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: gels <command> [options]");
        eprintln!();
        eprintln!("Commands:");
        eprintln!("  analyze <dir>              Detect traits, identify or synthesize grammar");
        eprintln!("  identify <dir>             Identify the language");
        eprintln!("  emit <dir> -o grammar.js   Output tree-sitter grammar");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "analyze" => {
            let dir = args.get(2).map(PathBuf::from).unwrap_or_else(|| ".".into());
            let source = read_sources(&dir);
            let results = gels::analyze(&source);
            for result in &results {
                if result.confidence.0 > 0.0 {
                    println!("{}: {:.2}", result.trait_id.0, result.confidence.0);
                }
            }
        }
        "identify" => {
            let dir = args.get(2).map(PathBuf::from).unwrap_or_else(|| ".".into());
            let source = read_sources(&dir);
            match gels::identify(&source) {
                Some(name) => println!("Identified: {name}"),
                None => println!("Unknown language"),
            }
        }
        "emit" => {
            let dir = args.get(2).map(PathBuf::from).unwrap_or_else(|| ".".into());
            let source = read_sources(&dir);
            let name = dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            let grammar = gels::synthesize(&source, name);
            let output = gels_synth::output::emit(&grammar);

            let out_path = args
                .windows(2)
                .find(|w| w[0] == "-o")
                .map(|w| PathBuf::from(&w[1]));

            match out_path {
                Some(path) => {
                    std::fs::write(&path, &output).expect("failed to write output file");
                    eprintln!("Wrote {}", path.display());
                }
                None => print!("{output}"),
            }
        }
        cmd => {
            eprintln!("Unknown command: {cmd}");
            std::process::exit(1);
        }
    }
}

fn read_sources(dir: &PathBuf) -> String {
    if dir.is_file() {
        return std::fs::read_to_string(dir).unwrap_or_default();
    }

    let mut combined = String::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && let Ok(content) = std::fs::read_to_string(&path)
            {
                combined.push_str(&content);
                combined.push('\n');
            }
        }
    }
    combined
}
