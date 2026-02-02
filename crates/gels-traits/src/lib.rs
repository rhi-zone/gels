pub mod bindings;
pub mod blocks;
pub mod delimiters;
pub mod keywords;
pub mod operators;
pub mod patterns;
pub mod registry;
pub mod strings;
pub mod terminators;
pub mod tokenize;

use gels_core::traits::SyntaxTrait;

pub fn register_all() -> Vec<Box<dyn SyntaxTrait>> {
    vec![
        Box::new(delimiters::BraceDelimited),
        Box::new(delimiters::ParenDelimited),
        Box::new(delimiters::BracketDelimited),
        Box::new(delimiters::AngleBracketDelimited),
        Box::new(terminators::SemicolonTerminated),
        Box::new(terminators::NewlineTerminated),
        Box::new(blocks::KeywordBlockDelimited),
        Box::new(blocks::IndentationScoped),
        Box::new(operators::InfixOperators),
        Box::new(operators::PrefixOperators),
        Box::new(bindings::AssignmentEquals),
        Box::new(bindings::AssignmentArrow),
        Box::new(bindings::TypeAnnotationColon),
        Box::new(bindings::TypeAnnotationPrefix),
        Box::new(bindings::MLLetBinding),
        Box::new(patterns::PatternMatching),
        Box::new(strings::SingleLineComment),
        Box::new(strings::MultiLineComment),
    ]
}
