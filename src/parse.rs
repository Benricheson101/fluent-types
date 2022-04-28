use fluent_syntax::ast::{
    CallArguments,
    Entry,
    Expression,
    InlineExpression,
    Pattern,
    PatternElement,
    Resource,
    Variant,
    VariantKey,
};

#[derive(Debug)]
pub struct ParsedMessages {
    pub msgs: Vec<ParsedMessage>,
}

impl ParsedMessages {
    pub fn to_typescript(&self) -> String {
        let mut parts = vec![];

        let header = "export type Messages = {";
        let msgs_as_ts = self
            .msgs
            .iter()
            .map(|m| {
                let ts = m.to_typescript();
                if let Some(comments) = &m.comments {
                    let comments = comments
                        .iter()
                        .map(|c| format!("   * {}", c))
                        .collect::<Vec<_>>()
                        .join("\n");

                    format!("  /**\n{}\n   */\n{}", comments, ts)
                } else {
                    ts
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        parts.push(header.into());
        parts.push(msgs_as_ts);
        parts.push("};".into());

        parts.join("\n")
    }
}

#[derive(Debug)]
pub struct ParsedMessage {
    pub name: String,
    pub comments: Option<Vec<String>>,
    pub placeholders: Vec<ParsedInlineExpr>,
}

impl ParsedMessage {
    pub fn to_typescript(&self) -> String {
        let mut parts = vec![];

        let header = format!("  '{}': {{", self.name);
        parts.push(header);

        if self.placeholders.len() > 0 {
            let placeholders = self
                .placeholders
                .iter()
                .map(|p| {
                    let default_types = "string | number | Date";
                    let typ = if let Some(variants) = &p.variants {
                        variants
                            .iter()
                            .map(|v| format!("'{}'", v))
                            .collect::<Vec<_>>()
                            .join(" | ")
                            + " | "
                            + default_types
                    } else {
                        default_types.into()
                    };

                    format!("    {}: {};", p.name, typ)
                })
                .collect::<Vec<_>>()
                .join("\n");

            parts.push(placeholders);
        };

        parts.push("  };".into());

        parts.join("\n")
    }
}

pub fn parse_resource(res: Resource<String>) -> ParsedMessages {
    walk_resource(res, 0)
}

fn walk_resource(resource: Resource<String>, depth: usize) -> ParsedMessages {
    let mut msgs = vec![];
    for node in &resource.body {
        match node {
            Entry::Message(msg) => {
                if let Some(pat) = &msg.value {
                    let p = walk_pattern(pat, depth + 1);

                    let m = ParsedMessage {
                        name: msg.id.name.clone(),
                        comments: msg.comment.clone().map(|c| c.content),
                        placeholders: p,
                    };

                    msgs.push(m);
                }
            },

            _ => {},
        }
    }

    msgs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    msgs.dedup_by(|a, b| a.name == b.name);

    ParsedMessages { msgs }
}

fn walk_pattern(
    pattern: &Pattern<String>,
    depth: usize,
) -> Vec<ParsedInlineExpr> {
    let mut elems = vec![];

    for elem in &pattern.elements {
        match elem {
            PatternElement::Placeable { expression: expr } => {
                let mut e = walk_placeable_expression(expr, depth + 1);
                elems.append(&mut e);
            },

            _ => {},
        }
    }

    elems
}

fn walk_placeable_expression(
    expr: &Expression<String>,
    depth: usize,
) -> Vec<ParsedInlineExpr> {
    let mut exprs = vec![];

    let depth = depth + 1;
    match expr {
        Expression::Select {
            selector: sel,
            variants: vars,
        } => {
            let variants = walk_variant(vars, depth + 1);
            let mut e = walk_inline_expr(sel, variants, depth + 1);

            exprs.append(&mut e);
        },

        Expression::Inline(inl) => {
            let mut e = walk_inline_expr(inl, None, depth + 1);
            exprs.append(&mut e);
        },
    };

    exprs
}

#[derive(Debug)]
pub struct ParsedInlineExpr {
    pub name: String,
    pub variants: Option<Vec<String>>,
}

fn walk_inline_expr(
    expr: &InlineExpression<String>,
    variants: Option<Vec<String>>,
    depth: usize,
) -> Vec<ParsedInlineExpr> {
    let mut exprs = vec![];

    match expr {
        InlineExpression::VariableReference { id } => {
            exprs.push(ParsedInlineExpr {
                name: id.name.clone(),
                variants,
            });
        },

        InlineExpression::Placeable { expression } => {
            // TODO: add to exprs
            walk_placeable_expression(expression, depth + 1);
        },

        InlineExpression::FunctionReference {
            id: _,
            arguments: args,
        } => {
            let mut args = walk_arguments(args, variants, depth + 1);
            exprs.append(&mut args);
        },
        _ => {},
    };

    exprs
}

fn walk_variant(
    vars: &Vec<Variant<String>>,
    depth: usize,
) -> Option<Vec<String>> {
    let mut variants = vec![];
    for var in vars {
        match &var.key {
            VariantKey::Identifier { name: id }
            | VariantKey::NumberLiteral { value: id } => {
                variants.push(id.to_owned())
            },
        }

        // TODO: patterns inside patterns
        walk_pattern(&var.value, depth + 1);
    }

    let ignore = vec!["1", "one", "two", "few", "many", "other"];

    let all_ignored = variants.iter().all(|v| ignore.contains(&v.as_str()));
    if all_ignored {
        return None;
    }

    let rmv = vec!["other"];

    let variants = variants
        .into_iter()
        .filter(|v| !rmv.contains(&v.as_str()))
        .collect::<Vec<_>>();

    if variants.is_empty() {
        None
    } else {
        Some(variants)
    }
}

fn walk_arguments(
    args: &CallArguments<String>,
    variants: Option<Vec<String>>,
    depth: usize,
) -> Vec<ParsedInlineExpr> {
    let mut exprs = vec![];

    for positional_arg in &args.positional {
        // TODO: can i do this without cloning?
        let mut e =
            walk_inline_expr(positional_arg, variants.clone(), depth + 2);
        exprs.append(&mut e);
    }

    for named_arg in &args.named {
        let mut e =
            walk_inline_expr(&named_arg.value, variants.clone(), depth + 2);
        exprs.append(&mut e);
    }

    exprs
}
