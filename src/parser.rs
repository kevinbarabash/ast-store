use std::sync::Arc;
use swc_atoms::JsWord;
use swc_common::{comments::SingleThreadedComments, FileName, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_parser::{
    lexer::Lexer, token::TokenAndSpan, Capturing, Parser, StringInput, Syntax, TsConfig,
};

use crate::ast;

pub fn parse(code: String) -> ast::Program {
    let comments = SingleThreadedComments::default();
    let cm = Arc::<SourceMap>::default();
    let fm = cm.new_source_file(FileName::Anon, code);

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: false,
            dts: true,
            decorators: false,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: false,
        }),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
    );
    let capturing = Capturing::new(lexer);
    let mut parser = Parser::new_from(capturing);

    let ast = parser.parse_module().expect("Failed to parse module.");

    convert_module(&ast, &parser.input().take())
}

fn convert_span(span: &swc_common::Span) -> ast::Range {
    (span.lo.0 - 1, span.hi.0 - 1)
}

// TODO: convert span to location
fn dummy_loc() -> ast::Location {
    ast::Location {
        start: ast::Position { line: 1, column: 0 },
        end: ast::Position { line: 1, column: 0 },
    }
}

fn convert_module(ast: &Module, tokens: &[TokenAndSpan]) -> ast::Program {
    for token in tokens {
        eprintln!("token: {:?}", token);
    }

    let tokens: Vec<ast::Token> = tokens
        .iter()
        .map(
            |TokenAndSpan {
                 token,
                 span,
                 had_line_break: _,
             }| match &token {
                swc_ecma_parser::token::Token::Word(word) => match word {
                    swc_ecma_parser::token::Word::Keyword(_) => {
                        ast::Token::Keyword(ast::TokenValue {
                            loc: dummy_loc(),
                            range: convert_span(span),
                            value: JsWord::from(word.clone()).to_string(),
                        })
                    }
                    swc_ecma_parser::token::Word::Null => todo!(),
                    swc_ecma_parser::token::Word::True => todo!(),
                    swc_ecma_parser::token::Word::False => todo!(),
                    swc_ecma_parser::token::Word::Ident(ident) => {
                        ast::Token::Identifier(ast::TokenValue {
                            loc: dummy_loc(),
                            range: convert_span(span),
                            value: ident.to_string(),
                        })
                    }
                },
                swc_ecma_parser::token::Token::Arrow => todo!(),
                swc_ecma_parser::token::Token::Hash => todo!(),
                swc_ecma_parser::token::Token::At => todo!(),
                swc_ecma_parser::token::Token::Dot => ast::Token::Punctuator(ast::TokenValue {
                    loc: dummy_loc(),
                    range: convert_span(span),
                    value: ".".to_string(),
                }),
                swc_ecma_parser::token::Token::DotDotDot => todo!(),
                swc_ecma_parser::token::Token::Bang => todo!(),
                swc_ecma_parser::token::Token::LParen => ast::Token::Punctuator(ast::TokenValue {
                    loc: dummy_loc(),
                    range: convert_span(span),
                    value: "(".to_string(),
                }),
                swc_ecma_parser::token::Token::RParen => ast::Token::Punctuator(ast::TokenValue {
                    loc: dummy_loc(),
                    range: convert_span(span),
                    value: ")".to_string(),
                }),
                swc_ecma_parser::token::Token::LBracket => {
                    ast::Token::Punctuator(ast::TokenValue {
                        loc: dummy_loc(),
                        range: convert_span(span),
                        value: "[".to_string(),
                    })
                }
                swc_ecma_parser::token::Token::RBracket => {
                    ast::Token::Punctuator(ast::TokenValue {
                        loc: dummy_loc(),
                        range: convert_span(span),
                        value: "]".to_string(),
                    })
                }
                swc_ecma_parser::token::Token::LBrace => ast::Token::Punctuator(ast::TokenValue {
                    loc: dummy_loc(),
                    range: convert_span(span),
                    value: "{".to_string(),
                }),
                swc_ecma_parser::token::Token::RBrace => ast::Token::Punctuator(ast::TokenValue {
                    loc: dummy_loc(),
                    range: convert_span(span),
                    value: "}".to_string(),
                }),
                swc_ecma_parser::token::Token::Semi => ast::Token::Punctuator(ast::TokenValue {
                    loc: dummy_loc(),
                    range: convert_span(span),
                    value: ";".to_string(),
                }),
                swc_ecma_parser::token::Token::Comma => ast::Token::Punctuator(ast::TokenValue {
                    loc: dummy_loc(),
                    range: convert_span(span),
                    value: ",".to_string(),
                }),
                swc_ecma_parser::token::Token::BackQuote => todo!(),
                swc_ecma_parser::token::Token::Template { raw: _, cooked: _ } => todo!(),
                swc_ecma_parser::token::Token::Colon => todo!(),
                swc_ecma_parser::token::Token::BinOp(_) => todo!(),
                swc_ecma_parser::token::Token::AssignOp(_) => todo!(),
                swc_ecma_parser::token::Token::DollarLBrace => todo!(),
                swc_ecma_parser::token::Token::QuestionMark => todo!(),
                swc_ecma_parser::token::Token::PlusPlus => todo!(),
                swc_ecma_parser::token::Token::MinusMinus => todo!(),
                swc_ecma_parser::token::Token::Tilde => todo!(),
                swc_ecma_parser::token::Token::Str { value: _, raw } => {
                    ast::Token::String(ast::TokenValue {
                        loc: dummy_loc(),
                        range: convert_span(span),
                        value: raw.to_string(),
                    })
                }
                swc_ecma_parser::token::Token::Regex(_, _) => todo!(),
                swc_ecma_parser::token::Token::Num { value: _, raw: _ } => todo!(),
                swc_ecma_parser::token::Token::BigInt { value: _, raw: _ } => todo!(),
                swc_ecma_parser::token::Token::JSXName { name: _ } => todo!(),
                swc_ecma_parser::token::Token::JSXText { raw: _ } => todo!(),
                swc_ecma_parser::token::Token::JSXTagStart => todo!(),
                swc_ecma_parser::token::Token::JSXTagEnd => todo!(),
                swc_ecma_parser::token::Token::Shebang(_) => todo!(),
                swc_ecma_parser::token::Token::Error(_) => todo!(),
            },
        )
        .collect();

    ast::Program {
        r#type: "Program".to_string(),
        loc: dummy_loc(),
        range: convert_span(&ast.span),
        body: ast.body.iter().map(convert_module_item).collect(),
        tokens,
        comments: vec![],
    }
}

fn convert_module_item(item: &ModuleItem) -> ast::Statement {
    match item {
        ModuleItem::ModuleDecl(_) => todo!(),
        ModuleItem::Stmt(stmt) => convert_statement(stmt),
    }
}

fn convert_statement(stmt: &Stmt) -> ast::Statement {
    match stmt {
        Stmt::Block(_) => todo!(),
        Stmt::Empty(_) => todo!(),
        Stmt::Debugger(DebuggerStmt { span }) => {
            ast::Statement::DebuggerStatement(ast::DebuggerStatement {
                loc: dummy_loc(),
                range: convert_span(span),
            })
        }
        Stmt::With(_) => todo!(),
        Stmt::Return(_) => todo!(),
        Stmt::Labeled(_) => todo!(),
        Stmt::Break(_) => todo!(),
        Stmt::Continue(_) => todo!(),
        Stmt::If(_) => todo!(),
        Stmt::Switch(_) => todo!(),
        Stmt::Throw(_) => todo!(),
        Stmt::Try(_) => todo!(),
        Stmt::While(_) => todo!(),
        Stmt::DoWhile(_) => todo!(),
        Stmt::For(_) => todo!(),
        Stmt::ForIn(_) => todo!(),
        Stmt::ForOf(_) => todo!(),
        Stmt::Decl(_) => todo!(),
        Stmt::Expr(ExprStmt { span, expr }) => {
            ast::Statement::ExpressionStatement(ast::ExpressionStatement {
                loc: dummy_loc(),
                range: convert_span(span),
                expression: Box::new(convert_expression(expr)),
            })
        }
    }
}

fn convert_expression(expr: &Expr) -> ast::Expression {
    match expr {
        Expr::This(_) => todo!(),
        Expr::Array(_) => todo!(),
        Expr::Object(_) => todo!(),
        Expr::Fn(_) => todo!(),
        Expr::Unary(_) => todo!(),
        Expr::Update(_) => todo!(),
        Expr::Bin(_) => todo!(),
        Expr::Assign(_) => todo!(),
        Expr::Member(MemberExpr { span, obj, prop }) => {
            ast::Expression::MemberExpression(ast::MemberExpression {
                loc: dummy_loc(),
                range: convert_span(span),
                object: Box::new(convert_expression(obj)),
                property: match prop {
                    MemberProp::Ident(ident) => {
                        Box::new(ast::Prop::Identifier(convert_ident(ident)))
                    }
                    MemberProp::PrivateName(_) => todo!(),
                    MemberProp::Computed(_) => todo!(),
                },
                computed: false, // TODO
            })
        }
        Expr::SuperProp(_) => todo!(),
        Expr::Cond(_) => todo!(),
        Expr::Call(CallExpr {
            span,
            callee: Callee::Expr(callee),
            args,
            type_args: _, // TODO
        }) => ast::Expression::CallExpression(ast::CallExpression {
            loc: dummy_loc(),
            range: convert_span(span),
            callee: Box::new(convert_expression(callee)),
            arguments: args
                .iter()
                .map(|arg| match arg {
                    ExprOrSpread { spread: None, expr } => {
                        ast::ExprOrSpread::Expr(convert_expression(expr))
                    }
                    ExprOrSpread {
                        spread: Some(span),
                        expr,
                    } => ast::ExprOrSpread::Spread(ast::SpreadElement {
                        loc: dummy_loc(),
                        range: convert_span(span),
                        argument: Box::new(convert_expression(expr)),
                    }),
                })
                .collect(),
        }),
        Expr::Call(_) => todo!(), // handle `super()` and `import()`
        Expr::New(_) => todo!(),
        Expr::Seq(_) => todo!(),
        Expr::Ident(ident) => ast::Expression::Identifier(convert_ident(ident)),
        Expr::Lit(lit) => match lit {
            Lit::Str(Str { span, value, raw }) => ast::Expression::Literal(ast::Literal {
                loc: dummy_loc(),
                range: convert_span(span),
                value: ast::Value::String(value.to_string()),
                raw: raw.as_ref().map(|s| s.to_string()),
            }),
            Lit::Bool(_) => todo!(),
            Lit::Null(_) => todo!(),
            Lit::Num(Number { span, value, raw }) => ast::Expression::Literal(ast::Literal {
                loc: dummy_loc(),
                range: convert_span(span),
                value: ast::Value::Number(value.to_owned()),
                raw: raw.as_ref().map(|s| s.to_string()),
            }),
            Lit::BigInt(_) => todo!(),
            Lit::Regex(_) => todo!(),
            Lit::JSXText(_) => todo!(),
        },
        Expr::Tpl(_) => todo!(),
        Expr::TaggedTpl(_) => todo!(),
        Expr::Arrow(_) => todo!(),
        Expr::Class(_) => todo!(),
        Expr::Yield(_) => todo!(),
        Expr::MetaProp(_) => todo!(),
        Expr::Await(_) => todo!(),
        Expr::Paren(_) => todo!(),
        Expr::JSXMember(_) => todo!(),
        Expr::JSXNamespacedName(_) => todo!(),
        Expr::JSXEmpty(_) => todo!(),
        Expr::JSXElement(_) => todo!(),
        Expr::JSXFragment(_) => todo!(),
        Expr::TsTypeAssertion(_) => todo!(),
        Expr::TsConstAssertion(_) => todo!(),
        Expr::TsNonNull(_) => todo!(),
        Expr::TsAs(_) => todo!(),
        Expr::TsInstantiation(_) => todo!(),
        Expr::TsSatisfies(_) => todo!(),
        Expr::PrivateName(_) => todo!(),
        Expr::OptChain(_) => todo!(),
        Expr::Invalid(_) => todo!(),
    }
}

fn convert_ident(ident: &Ident) -> ast::Identifier {
    ast::Identifier {
        loc: dummy_loc(),
        range: convert_span(&ident.span),
        name: ident.sym.to_string(),
    }
}
