use gels_core::grammar::Grammar;
use gels_core::traits::{Confidence, DetectionResult};

/// Analyze source files: tokenize, detect traits, return detection results.
pub fn analyze(source: &str) -> Vec<DetectionResult> {
    let tokens = gels_traits::tokenize::tokenize(source);
    let traits = gels_traits::register_all();
    traits.iter().map(|t| t.detect(&tokens)).collect()
}

/// Identify the language from source files by matching against known profiles.
pub fn identify(source: &str) -> Option<String> {
    let detections = analyze(source);
    let profiles = gels_traits::registry::known_profiles();

    // Score each profile against the detections
    let mut best: Option<(String, f64)> = None;
    for profile in &profiles {
        let mut score = 0.0;
        for (trait_id, expected) in &profile.traits {
            if let Some(det) = detections.iter().find(|d| d.trait_id == *trait_id) {
                score += det.confidence.0 * expected.0;
            }
        }
        if best.as_ref().is_none_or(|(_, s)| score > *s) {
            best = Some((profile.name.clone(), score));
        }
    }

    best.filter(|(_, score)| *score > 0.5).map(|(name, _)| name)
}

/// Synthesize a tree-sitter grammar from detected traits.
pub fn synthesize(source: &str, name: &str) -> Grammar {
    let tokens = gels_traits::tokenize::tokenize(source);
    let traits = gels_traits::register_all();

    let detections: Vec<_> = traits
        .iter()
        .map(|t| {
            let result = t.detect(&tokens);
            let fragment = t.grammar_fragment();
            (result, fragment)
        })
        .collect();

    let mut grammar = gels_synth::merge::merge_fragments(&detections, Confidence(0.3));
    grammar.name = name.to_string();
    grammar
}
