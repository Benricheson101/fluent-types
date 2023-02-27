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

#[derive(Debug, Clone)]
pub struct ParsedMessages {
    pub msgs: Vec<ParsedMessage>,
}

#[derive(Debug, Clone)]
pub struct ParsedAttribute {
    pub name: String,
    pub placeholders: Option<Vec<ParsedInlineExpr>>,
}

#[derive(Debug, Clone)]
pub struct ParsedMessage {
    pub name: String,
    pub comments: Option<Vec<String>>,
    pub placeholders: Option<Vec<ParsedInlineExpr>>,
    pub attrs: Option<Vec<ParsedAttribute>>,
    pub has_value: bool,
}

#[derive(Debug, Clone)]
pub struct ParsedInlineExpr {
    pub name: String,
    pub variants: Option<Vec<String>>,
}

pub fn parse_resource(res: Resource<String>) -> ParsedMessages {
    walk_resource(res, 0)
}

fn walk_resource(resource: Resource<String>, depth: usize) -> ParsedMessages {
    let mut msgs = vec![];
    for node in &resource.body {
        match node {
            Entry::Message(msg) => {
                let mut m = ParsedMessage {
                    name: msg.id.name.clone(),
                    has_value: false,
                    placeholders: None,
                    attrs: None,
                    comments: None,
                };

                if let Some(pat) = &msg.value {
                    let p = walk_pattern(pat, depth + 1);

                    m.comments = msg.comment.clone().map(|c| c.content);
                    m.placeholders = Some(p);
                    m.has_value = true;
                }

                let parsed_attrs = msg
                    .attributes
                    .iter()
                    .map(|a| {
                        let p = walk_pattern(&a.value, depth + 1);

                        ParsedAttribute {
                            name: a.id.name.clone(),
                            placeholders: if p.is_empty() {
                                None
                            } else {
                                Some(p)
                            },
                        }
                    })
                    .collect::<Vec<_>>();

                if !parsed_attrs.is_empty() {
                    m.attrs = Some(parsed_attrs);
                }

                msgs.push(m);
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
