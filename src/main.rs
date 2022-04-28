use std::{error, fs};

use fluent_syntax::{
    ast::{
        Entry,
        Expression,
        InlineExpression,
        Pattern,
        PatternElement,
        Resource,
        Variant,
        VariantKey,
    },
    parser,
};

const DBG_SPACES: usize = 2;

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::read_to_string("lang.ftl")?;

    let p = parser::parse(file).unwrap();
    walk_resource(p, 0);

    Ok(())
}

fn walk_resource(resource: Resource<String>, depth: usize) {
    for node in &resource.body {
        match node {
            Entry::Message(msg) => {
                println!(
                    "{:width$}Message: {:?}",
                    "",
                    msg.id.name,
                    width = DBG_SPACES * depth
                );

                if let Some(pat) = &msg.value {
                    walk_pattern(pat, depth + 1);
                }
            },

            Entry::Comment(comment)
            | Entry::GroupComment(comment)
            | Entry::ResourceComment(comment) => {
                println!(
                    "{:width$}Comment: {:?}",
                    "",
                    comment.content,
                    width = DBG_SPACES * depth
                )
            },

            _ => {
                println!("found not message");
            },
        }
    }
}

fn walk_pattern(pattern: &Pattern<String>, depth: usize) {
    for elem in &pattern.elements {
        match elem {
            PatternElement::TextElement { value: text } => {
                println!(
                    "{:width$}TextElement: {:?}",
                    "",
                    text,
                    width = DBG_SPACES * depth
                );
            },

            PatternElement::Placeable { expression: expr } => {
                // println!("{:width$}Placeable:", "", width = DBG_SPACES *
                // depth);
                walk_placeable_expression(expr, depth + 1)
            },
        }
    }
}

fn walk_placeable_expression(expr: &Expression<String>, depth: usize) {
    println!("{:width$}Placeable:", "", width = DBG_SPACES * depth);
    let depth = depth + 1;
    match expr {
        Expression::Select {
            selector: sel,
            variants: vars,
        } => {
            println!("{:width$}Selector:", "", width = DBG_SPACES * depth);
            walk_inline_expr(sel, depth + 1);
            walk_variant(vars, depth + 1);
        },

        Expression::Inline(inl) => {
            println!("{:width$}Inline:", "", width = DBG_SPACES * depth);
            walk_inline_expr(inl, depth + 1);
        },
    }
}

fn walk_inline_expr(expr: &InlineExpression<String>, depth: usize) {
    match expr {
        InlineExpression::VariableReference { id } => {
            println!(
                "{:width$}VariableReference: {:?}",
                "",
                id.name,
                width = DBG_SPACES * depth
            );
        },

        InlineExpression::Placeable { expression } => {
            walk_placeable_expression(expression, depth + 1);
        },

        _ => {
            println!("{:width$}Other", "", width = DBG_SPACES * depth);
        },
    }
}

fn walk_variant(vars: &Vec<Variant<String>>, depth: usize) {
    for var in vars {
        match &var.key {
            VariantKey::Identifier { name } => {
                println!(
                    "{:width$}Variant: {:?}",
                    "",
                    name,
                    width = DBG_SPACES * depth
                );
            },

            VariantKey::NumberLiteral { value } => {
                println!(
                    "{:width$}Variant: {:?}",
                    "",
                    value,
                    width = DBG_SPACES * depth
                );
            },
        }

        walk_pattern(&var.value, depth + 1);
    }
}
