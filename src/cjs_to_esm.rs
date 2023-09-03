use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;

pub fn cjs_to_esm(module: &mut Module) {
    for item in module.body.iter_mut() {
        if let ModuleItem::Stmt(Stmt::Expr(ExprStmt { span: _, expr })) = item {
            if let Expr::Assign(AssignExpr {
                op: AssignOp::Assign,
                left: PatOrExpr::Pat(left),
                right,
                ..
            }) = expr.as_mut()
            {
                if is_module_exports(left.as_ref()) {
                    *item =
                        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
                            span: DUMMY_SP,
                            expr: right.to_owned(),
                        }));

                    continue;
                }

                if let Some(prop) = is_exports_member(left) {
                    let new_item = match right.as_ref() {
                        Expr::Fn(FnExpr { ident: _, function }) => {
                            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                                span: DUMMY_SP,
                                decl: Decl::Fn(FnDecl {
                                    ident: Ident::new(prop, DUMMY_SP),
                                    declare: false,
                                    function: function.to_owned(),
                                }),
                            }))
                        }
                        Expr::Class(ClassExpr { ident: _, class }) => {
                            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                                span: DUMMY_SP,
                                decl: Decl::Class(ClassDecl {
                                    ident: Ident::new(prop, DUMMY_SP),
                                    declare: false,
                                    class: class.to_owned(),
                                }),
                            }))
                        }
                        Expr::Ident(ident) => {
                            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(NamedExport {
                                span: DUMMY_SP,
                                specifiers: vec![ExportSpecifier::Named(ExportNamedSpecifier {
                                    span: DUMMY_SP,
                                    orig: ModuleExportName::Ident(Ident::new(prop, DUMMY_SP)),
                                    exported: Some(ModuleExportName::Ident(ident.to_owned())),
                                    is_type_only: false,
                                })],
                                src: None,
                                type_only: false,
                                with: None,
                            }))
                        }
                        _ => ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                            span: DUMMY_SP,
                            decl: Decl::Var(Box::new(VarDecl {
                                span: DUMMY_SP,
                                kind: VarDeclKind::Const,
                                declare: false,
                                decls: vec![VarDeclarator {
                                    span: DUMMY_SP,
                                    name: Pat::Ident(BindingIdent {
                                        id: Ident::new(prop, DUMMY_SP),
                                        type_ann: None,
                                    }),
                                    definite: false,
                                    init: Some(right.to_owned()),
                                }],
                            })),
                        })),
                    };

                    *item = new_item;

                    continue;
                }

                eprintln!("right = {:?}", right.as_ref());

                if let Expr::Call(CallExpr {
                    callee: Callee::Expr(callee),
                    args,
                    ..
                }) = right.as_ref()
                {
                    eprintln!("callee = {:?}", callee);
                    match callee.as_ref() {
                        Expr::Ident(Ident { sym, .. }) if sym == "require" => {
                            // TODO: extract the source from the callee
                            // TODO: determine whether this is a default import or one-or-more named imports
                            let arg = &args[0];

                            eprintln!("arg = {:?}", arg);

                            let src = match arg {
                                ExprOrSpread { spread: None, expr } => match expr.as_ref() {
                                    Expr::Lit(Lit::Str(src)) => src,
                                    _ => panic!("TODO: handle non-string require() arguments"),
                                },
                                _ => panic!("TODO: handle non-string require() arguments"),
                            };

                            match left.as_ref() {
                                Pat::Ident(ident) => {
                                    let new_item =
                                        ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                                            span: DUMMY_SP,
                                            specifiers: vec![ImportSpecifier::Default(
                                                ImportDefaultSpecifier {
                                                    span: DUMMY_SP,
                                                    local: ident.id.to_owned(),
                                                },
                                            )],
                                            src: Box::new(src.to_owned()),
                                            type_only: false,
                                            with: None,
                                        }));

                                    *item = new_item;
                                }
                                Pat::Array(_) => todo!(),
                                Pat::Rest(_) => todo!(),
                                Pat::Object(_) => {
                                    // maybe named imports
                                    todo!()
                                }
                                Pat::Assign(_) => todo!(),
                                Pat::Invalid(_) => todo!(),
                                Pat::Expr(_) => todo!(),
                            }
                        }
                        _ => (),
                    };

                    continue;
                }
            }
        }

        if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(decl))) = item {
            let VarDecl { decls, .. } = decl.as_ref();

            // TODO: handle multiple decls
            let decl = &decls[0];

            if let Some(init) = &decl.init {
                if let Expr::Call(CallExpr {
                    callee: Callee::Expr(callee),
                    args,
                    ..
                }) = init.as_ref()
                {
                    if let Expr::Ident(Ident { sym, .. }) = callee.as_ref() {
                        if sym == "require" {
                            let src = match &args[0] {
                                ExprOrSpread { spread: None, expr } => match expr.as_ref() {
                                    Expr::Lit(Lit::Str(src)) => src,
                                    _ => panic!("TODO: handle non-string require() arguments"),
                                },
                                _ => panic!("TODO: handle non-string require() arguments"),
                            };

                            match &decl.name {
                                Pat::Ident(ident) => {
                                    let new_item =
                                        ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                                            span: DUMMY_SP,
                                            specifiers: vec![ImportSpecifier::Default(
                                                ImportDefaultSpecifier {
                                                    span: DUMMY_SP,
                                                    local: ident.id.to_owned(),
                                                },
                                            )],
                                            src: Box::new(src.to_owned()),
                                            type_only: false,
                                            with: None,
                                        }));

                                    *item = new_item;
                                }
                                Pat::Array(_) => todo!(),
                                Pat::Rest(_) => todo!(),
                                Pat::Object(ObjectPat { props, .. }) => {
                                    let mut specifiers = vec![];

                                    for prop in props {
                                        // TODO: handle deep destructuring
                                        match prop {
                                            ObjectPatProp::Assign(AssignPatProp {
                                                span,
                                                key,
                                                value: None,
                                            }) => {
                                                specifiers.push(ImportSpecifier::Named(
                                                    ImportNamedSpecifier {
                                                        span: span.to_owned(),
                                                        imported: None,
                                                        local: key.to_owned(),
                                                        is_type_only: false,
                                                    },
                                                ));
                                            }
                                            ObjectPatProp::KeyValue(KeyValuePatProp {
                                                key: PropName::Ident(key),
                                                value: box Pat::Ident(value),
                                            }) => {
                                                specifiers.push(ImportSpecifier::Named(
                                                    ImportNamedSpecifier {
                                                        span: DUMMY_SP,
                                                        imported: Some(ModuleExportName::Ident(
                                                            key.to_owned(),
                                                        )),
                                                        local: value.id.to_owned(),
                                                        is_type_only: false,
                                                    },
                                                ));
                                            }
                                            ObjectPatProp::KeyValue(KeyValuePatProp {
                                                key: PropName::Str(key),
                                                value: box Pat::Ident(value),
                                            }) => {
                                                specifiers.push(ImportSpecifier::Named(
                                                    ImportNamedSpecifier {
                                                        span: DUMMY_SP,
                                                        imported: Some(ModuleExportName::Str(
                                                            key.to_owned(),
                                                        )),
                                                        local: value.id.to_owned(),
                                                        is_type_only: false,
                                                    },
                                                ));
                                            }
                                            ObjectPatProp::Rest(_) => todo!(),
                                            _ => todo!(),
                                        }
                                    }

                                    let new_item =
                                        ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                                            span: DUMMY_SP,
                                            specifiers,
                                            src: Box::new(src.to_owned()),
                                            type_only: false,
                                            with: None,
                                        }));

                                    *item = new_item;
                                }
                                Pat::Assign(_) => todo!(),
                                Pat::Invalid(_) => todo!(),
                                Pat::Expr(_) => todo!(),
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_module_exports(pat: &Pat) -> bool {
    match pat {
        Pat::Expr(expr) => match expr.as_ref() {
            Expr::Member(MemberExpr { span: _, obj, prop }) => match obj.as_ref() {
                Expr::Ident(Ident { sym: obj, .. }) => match prop {
                    MemberProp::Ident(Ident { sym: prop, .. }) => {
                        obj == "module" && prop == "exports"
                    }
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

fn is_exports_member(pat: &Pat) -> Option<JsWord> {
    match pat {
        Pat::Expr(expr) => match expr.as_ref() {
            Expr::Member(MemberExpr { span: _, obj, prop }) => match obj.as_ref() {
                Expr::Ident(Ident { sym: obj, .. }) => match prop {
                    MemberProp::Ident(Ident { sym: prop, .. }) => {
                        if obj == "exports" {
                            Some(prop.to_owned())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn is_require_call(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(ExprStmt { span: _, expr }) => match expr.as_ref() {
            Expr::Call(CallExpr {
                callee: Callee::Expr(callee),
                ..
            }) => match callee.as_ref() {
                Expr::Ident(Ident { sym, .. }) => sym == "require",
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::Arc;

    use swc_common::source_map::{self, FilePathMapping};
    use swc_common::{comments::SingleThreadedComments, FileName, SourceMap};
    use swc_ecma_ast::*;
    use swc_ecma_codegen::*;
    use swc_ecma_parser::{error::Error, parse_file_as_module, Syntax, TsConfig};
    use swc_ecma_visit::*;

    use super::cjs_to_esm;

    pub fn parse(input: &str) -> Module {
        let cm = Arc::<SourceMap>::default();
        let fm = cm.new_source_file(FileName::Anon, input.to_string());

        let mut errors: Vec<Error> = vec![];
        let comments = SingleThreadedComments::default();

        parse_file_as_module(
            &fm,
            Syntax::Typescript(TsConfig {
                tsx: true,
                dts: false,
                decorators: false,
                no_early_errors: false,
                disallow_ambiguous_jsx_like: false,
            }),
            EsVersion::Es2020,
            Some(&comments),
            &mut errors,
        )
        .unwrap()
    }

    pub fn print(module: &Module) -> String {
        let mut buf = vec![];
        let mut src_map = vec![];
        let cm = Rc::new(source_map::SourceMap::new(FilePathMapping::empty()));

        cm.new_source_file(FileName::Anon, String::from(""));

        let wr = text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, Some(&mut src_map));
        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config {
                ..Default::default()
            },
            cm,
            comments: None,
            wr,
        };
        emitter.emit_module(module).unwrap();

        String::from_utf8_lossy(&buf).to_string()
    }

    pub fn transform(input: &str) -> String {
        let mut module = parse(input);
        cjs_to_esm(&mut module);
        print(&module)
    }

    #[test]
    fn function_export() {
        let input = r#"exports.foo = function foo () { return "foo"; };"#;
        let output = transform(input);

        insta::assert_snapshot!(output, @r###"
        export function foo() {
            return "foo";
        }
        "###);
    }

    #[test]
    fn class_export() {
        let input = r#"exports.Foo = class { static bar = "bar" };"#;
        let output = transform(input);

        insta::assert_snapshot!(output, @r###"
        export class Foo {
            static bar = "bar";
        }
        "###);
    }

    #[test]
    fn identifier_export() {
        let input = r#"exports.foo = bar;"#;
        let output = transform(input);

        insta::assert_snapshot!(output, @"export { foo as bar };
");
    }

    #[test]
    fn expression_export() {
        let input = r#"exports.foo = "bar";"#;
        let output = transform(input);

        insta::assert_snapshot!(output, @r###"export const foo = "bar";
"###);
    }

    #[test]
    fn default_function_export() {
        let input = r#"module.exports = function foo () { return "foo"; };"#;
        let output = transform(input);

        insta::assert_snapshot!(output, @r###"
        export default function foo() {
            return "foo";
        };
        "###);
    }

    #[test]
    fn default_class_export() {
        let input = r#"
            module.exports = class Foo { 
                render() { 
                    return <h1>Hello, world!</h1>;
                } 
            };
        "#;
        let output = transform(input);

        insta::assert_snapshot!(output, @r###"
        export default class Foo {
            render() {
                return <h1>Hello, world!</h1>;
            }
        };
        "###);
    }

    #[test]
    fn default_import() {
        let input = r#"const foo = require("./foo");"#;
        let output = transform(input);

        insta::assert_snapshot!(output, @r###"import foo from "./foo";
"###);
    }

    #[test]
    fn named_import() {
        let input = r#"const {foo, bar: baz} = require("./foo-bar");"#;
        let output = transform(input);

        insta::assert_snapshot!(output, @r###"import { foo, bar as baz } from "./foo-bar";
"###);
    }
}
