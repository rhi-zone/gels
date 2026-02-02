use crate::grammar::Grammar;
use crate::traits::{Confidence, TraitId};

#[derive(Debug, Clone)]
pub struct LanguageProfile {
    pub name: String,
    pub traits: Vec<(TraitId, Confidence)>,
    pub grammar: Grammar,
}
