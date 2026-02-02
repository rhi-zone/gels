use crate::grammar::{Grammar, Precedence, Rule};
use std::fmt::Write;

pub fn emit_grammar_js(grammar: &Grammar) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "/// @ts-nocheck");
    let _ = writeln!(out, "module.exports = grammar({{");
    let _ = writeln!(out, "  name: {:?},", grammar.name);
    let _ = writeln!(out);

    // Rules
    let _ = writeln!(out, "  rules: {{");
    for (name, rule) in &grammar.rules {
        let _ = writeln!(out, "    {name}: $ => {},", emit_rule(rule));
    }
    let _ = writeln!(out, "  }},");

    // Extras
    if !grammar.extras.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "  extras: $ => [");
        for rule in &grammar.extras {
            let _ = writeln!(out, "    {},", emit_rule(rule));
        }
        let _ = writeln!(out, "  ],");
    }

    // Conflicts
    if !grammar.conflicts.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "  conflicts: $ => [");
        for conflict in &grammar.conflicts {
            let names: Vec<_> = conflict.iter().map(|n| format!("$.{n}")).collect();
            let _ = writeln!(out, "    [{}],", names.join(", "));
        }
        let _ = writeln!(out, "  ],");
    }

    let _ = writeln!(out, "}});");
    out
}

fn emit_rule(rule: &Rule) -> String {
    match rule {
        Rule::String(s) => format!("{s:?}"),
        Rule::Pattern(p) => format!("/{p}/"),
        Rule::Symbol(name) => format!("$.{name}"),
        Rule::Seq(rules) => {
            let parts: Vec<_> = rules.iter().map(emit_rule).collect();
            format!("seq({})", parts.join(", "))
        }
        Rule::Choice(rules) => {
            let parts: Vec<_> = rules.iter().map(emit_rule).collect();
            format!("choice({})", parts.join(", "))
        }
        Rule::Repeat(rule) => format!("repeat({})", emit_rule(rule)),
        Rule::Repeat1(rule) => format!("repeat1({})", emit_rule(rule)),
        Rule::Optional(rule) => format!("optional({})", emit_rule(rule)),
        Rule::Prec(prec, rule) => {
            let inner = emit_rule(rule);
            match prec {
                Precedence::None => inner,
                Precedence::Left(n) => format!("prec.left({n}, {inner})"),
                Precedence::Right(n) => format!("prec.right({n}, {inner})"),
                Precedence::Dynamic(n) => format!("prec.dynamic({n}, {inner})"),
            }
        }
        Rule::Token(rule) => format!("token({})", emit_rule(rule)),
        Rule::Field(name, rule) => format!("field({name:?}, {})", emit_rule(rule)),
    }
}
