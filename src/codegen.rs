use std::{collections::HashMap, sync::Arc};

use swc::{atoms::JsWord, config::SourceMapsConfig, Compiler};
use swc_common::{SourceMap, DUMMY_SP, GLOBALS};
use swc_ecma_ast::{
    Bool,
    Decl,
    EsVersion,
    ExportDecl,
    Expr,
    Ident,
    Module,
    ModuleDecl,
    ModuleItem,
    Program,
    Str,
    TsEntityName,
    TsKeywordType,
    TsKeywordTypeKind,
    TsLit,
    TsLitType,
    TsPropertySignature,
    TsType,
    TsTypeAliasDecl,
    TsTypeAnn,
    TsTypeElement,
    TsTypeLit,
    TsTypeRef,
    TsUnionOrIntersectionType,
    TsUnionType,
};

use crate::parse::{
    ParsedAttribute,
    ParsedInlineExpr,
    ParsedMessage,
    ParsedMessages,
};

pub fn generate_ts(parsed: &ParsedMessages) -> String {
    let cm = Arc::new(SourceMap::default());

    let output = GLOBALS.set(&Default::default(), || {
        let placeholder_val_type = ModuleDecl::ExportDecl(ExportDecl {
            span: DUMMY_SP,
            decl: Decl::TsTypeAlias(Box::new(TsTypeAliasDecl {
                span: DUMMY_SP,
                type_params: None,
                declare: false,
                id: Ident::new(JsWord::from("FluentPlaceholder"), DUMMY_SP),
                type_ann: Box::new(placeholder_union()),
            })),
        });

        let export_messages_type = ModuleDecl::ExportDecl(ExportDecl {
            span: DUMMY_SP,
            decl: Decl::TsTypeAlias(Box::new(TsTypeAliasDecl {
                span: DUMMY_SP,
                id: Ident::new(JsWord::from("Messages"), DUMMY_SP),
                declare: false,
                type_params: None,
                type_ann: Box::new(TsType::TsTypeLit(TsTypeLit {
                    span: DUMMY_SP,
                    members: parsed
                        .msgs
                        .iter()
                        .map(build_message_member)
                        .collect(),
                })),
            })),
        });

        let prog = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![
                ModuleItem::ModuleDecl(placeholder_val_type),
                ModuleItem::ModuleDecl(export_messages_type),
            ],
            shebang: None,
        });

        Compiler::new(cm.clone())
            .print(
                &prog,
                None,
                None,
                false,
                EsVersion::Es2022,
                SourceMapsConfig::Bool(false),
                &HashMap::default(),
                None,
                false,
                None,
                false,
                false,
            )
            .unwrap()
    });

    let code = output.code;

    const UTILITY_TYPES: &'static str = include_str!("../utilityTypes.ts");
    format!("/* prettier-ignore */\n// eslint-ignore\n{code}\n{UTILITY_TYPES}").trim().to_string()
}

fn build_message_member(msg: &ParsedMessage) -> TsTypeElement {
    let type_ann = TsTypeAnn {
        span: DUMMY_SP,
        type_ann: Box::new(TsType::TsTypeLit(TsTypeLit {
            span: DUMMY_SP,
            members: vec![
                build_has_value_member(msg.has_value),
                build_attribute_map(msg.attrs.as_ref()),
                build_placeholder_map(msg.placeholders.as_ref()),
            ],
        })),
    };

    let ty = property_signature(
        &quote_str(msg.name.as_str()),
        Some(Box::new(type_ann)),
    );

    ty
}

fn build_has_value_member(has_value: bool) -> TsTypeElement {
    let type_ann = TsTypeAnn {
        span: DUMMY_SP,
        type_ann: Box::new(TsType::TsLitType(TsLitType {
            span: DUMMY_SP,
            lit: TsLit::Bool(Bool {
                span: DUMMY_SP,
                value: has_value,
            }),
        })),
    };

    property_signature("hasValue", Some(Box::new(type_ann)))
}

fn build_attribute_map(attrs: Option<&Vec<ParsedAttribute>>) -> TsTypeElement {
    let members = match attrs {
        Some(attrs) => attrs.iter().map(build_attribute_map_member).collect(),
        None => vec![],
    };

    let type_ann = TsTypeAnn {
        span: DUMMY_SP,
        type_ann: Box::new(TsType::TsTypeLit(TsTypeLit {
            span: DUMMY_SP,
            members,
        })),
    };

    let ty = property_signature("attributes", Some(Box::new(type_ann)));
    ty
}

fn build_attribute_map_member(attr: &ParsedAttribute) -> TsTypeElement {
    let ty = TsTypeAnn {
        span: DUMMY_SP,
        type_ann: Box::new(TsType::TsTypeLit(TsTypeLit {
            span: DUMMY_SP,
            members: vec![build_placeholder_map(attr.placeholders.as_ref())],
        })),
    };

    property_signature(&quote_str(attr.name.as_str()), Some(Box::new(ty)))
}

fn build_placeholder_map(
    placeholders: Option<&Vec<ParsedInlineExpr>>,
) -> TsTypeElement {
    let members = placeholders
        .and_then(|ph| {
            Some(
                ph.iter()
                    .map(build_placeholder_map_member)
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or_else(Vec::new);

    let type_ann = TsTypeAnn {
        span: DUMMY_SP,
        type_ann: Box::new(TsType::TsTypeLit(TsTypeLit {
            span: DUMMY_SP,
            members,
        })),
    };

    property_signature("placeholders", Some(Box::new(type_ann)))
}

fn build_placeholder_map_member(ph: &ParsedInlineExpr) -> TsTypeElement {
    let fluent_placeholder_ref = TsType::TsTypeRef(TsTypeRef {
        span: DUMMY_SP,
        type_params: None,
        type_name: TsEntityName::Ident(Ident::new(
            JsWord::from("FluentPlaceholder"),
            DUMMY_SP,
        )),
    });

    let ty = match &ph.variants {
        Some(v) => TsType::TsUnionOrIntersectionType(
            TsUnionOrIntersectionType::TsUnionType(TsUnionType {
                span: DUMMY_SP,
                types: {
                    let mut variants: Vec<_> = v
                        .iter()
                        .map(|v| {
                            Box::new(TsType::TsLitType(TsLitType {
                                span: DUMMY_SP,
                                lit: TsLit::Str(Str::from(v.as_str())),
                            }))
                        })
                        .collect();

                    variants.push(Box::new(fluent_placeholder_ref));

                    variants
                },
            }),
        ),
        None => fluent_placeholder_ref,
    };

    let type_ann = TsTypeAnn {
        span: DUMMY_SP,
        type_ann: Box::new(ty),
    };

    property_signature(&quote_str(ph.name.as_str()), Some(Box::new(type_ann)))
}

fn property_signature(
    name: &str,
    type_ann: Option<Box<TsTypeAnn>>,
) -> TsTypeElement {
    TsTypeElement::TsPropertySignature(TsPropertySignature {
        span: DUMMY_SP,
        key: Box::new(Expr::Ident(Ident::new(JsWord::from(name), DUMMY_SP))),
        type_params: None,
        init: None,
        params: vec![],
        readonly: true,
        computed: false,
        optional: false,
        type_ann,
    })
}

fn placeholder_union() -> TsType {
    let string_kw = Box::new(TsType::TsKeywordType(TsKeywordType {
        span: DUMMY_SP,
        kind: TsKeywordTypeKind::TsStringKeyword,
    }));

    let number_kw = Box::new(TsType::TsKeywordType(TsKeywordType {
        span: DUMMY_SP,
        kind: TsKeywordTypeKind::TsNumberKeyword,
    }));

    let date_ty = Box::new(TsType::TsTypeRef(TsTypeRef {
        span: DUMMY_SP,
        type_params: None,
        type_name: TsEntityName::Ident(Ident::new(
            JsWord::from("Date"),
            DUMMY_SP,
        )),
    }));

    TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(
        TsUnionType {
            span: DUMMY_SP,
            types: vec![string_kw, number_kw, date_ty],
        },
    ))
}

fn quote_str(s: &str) -> String {
    format!("{quote}{s}{quote}", quote = '"')
}
