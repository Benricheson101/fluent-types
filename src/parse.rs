use std::env;

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

const DBG_SPACES: usize = 2;

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
                    let typ = if let Some(variants) = &p.variants {
                        variants
                            .iter()
                            .map(|v| format!("'{}'", v))
                            .collect::<Vec<_>>()
                            .join(" | ")
                            + " | string"
                    } else {
                        "string".into()
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

macro_rules! print_tree_node {
    ($name:expr, $width:expr) => {
        {
            if (env::var("APP_ENV").unwrap_or("dev".into()) == "dev"){
                println!("{:width$}{}", "", $name, width=$width*DBG_SPACES);
            }
        }
    };

    ($name:expr, $width:expr, $($arg:expr),*$(,)?) => {
        {
            if (env::var("APP_ENV").unwrap_or("dev".into()) == "dev"){
                print!("{:width$}{}", "", $name, width=$width*DBG_SPACES);
                $(print!(" {:?}", $arg);)*
                print!("\n");
            }
        }
    };
}

pub fn parse_resource(res: Resource<String>) -> ParsedMessages {
    walk_resource(res, 0)
}

fn walk_resource(resource: Resource<String>, depth: usize) -> ParsedMessages {
    let mut msgs = vec![];
    for node in &resource.body {
        match node {
            Entry::Message(msg) => {
                print_tree_node!("Message", depth, msg.id.name);

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

            Entry::Comment(comment) => {
                print_tree_node!("Comment", depth, comment.content);
            },

            Entry::Junk { content } => {
                print_tree_node!("Junk", depth, content);
            },

            Entry::Term(term) => {
                print_tree_node!("Term", depth, term.id.name);
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
            PatternElement::TextElement { value: text } => {
                print_tree_node!("TextElement", depth, text);
            },

            PatternElement::Placeable { expression: expr } => {
                let mut e = walk_placeable_expression(expr, depth + 1);
                elems.append(&mut e);
            },
        }
    }

    elems
}

fn walk_placeable_expression(
    expr: &Expression<String>,
    depth: usize,
) -> Vec<ParsedInlineExpr> {
    let mut exprs = vec![];

    print_tree_node!("Placeable", depth);
    let depth = depth + 1;
    match expr {
        Expression::Select {
            selector: sel,
            variants: vars,
        } => {
            print_tree_node!("Selector", depth);
            let variants = walk_variant(vars, depth + 1);
            let mut e = walk_inline_expr(sel, variants, depth + 1);

            exprs.append(&mut e);
        },

        Expression::Inline(inl) => {
            print_tree_node!("InlineExpression", depth);
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
            print_tree_node!("VariableReference", depth, id.name);

            exprs.push(ParsedInlineExpr {
                name: id.name.clone(),
                variants,
            });
        },

        InlineExpression::Placeable { expression } => {
            walk_placeable_expression(expression, depth + 1);
        },

        InlineExpression::FunctionReference {
            id,
            arguments: args,
        } => {
            print_tree_node!("FunctionReference", depth, id.name);
            let mut args = walk_arguments(args, variants, depth + 1);

            exprs.append(&mut args);
        },

        InlineExpression::StringLiteral { value }
        | InlineExpression::NumberLiteral { value } => {
            print_tree_node!("Literal", depth, value);
        },

        _ => {
            print_tree_node!("OtherInlineType", depth);
        },
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
                print_tree_node!("Variant", depth, id,);
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
    print_tree_node!("CallArguments", depth);

    let mut exprs = vec![];

    for positional_arg in &args.positional {
        print_tree_node!("PositionalArg", depth + 1);
        // TODO: can i do this without cloning?
        let mut e =
            walk_inline_expr(positional_arg, variants.clone(), depth + 2);
        exprs.append(&mut e);
    }

    for named_arg in &args.named {
        print_tree_node!("NamedArg", depth + 1, named_arg.name.name);
        let mut e =
            walk_inline_expr(&named_arg.value, variants.clone(), depth + 2);
        exprs.append(&mut e);
    }

    exprs
}
