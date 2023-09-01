use std::error;
use std::sync::Arc;
use swc_common::{comments::SingleThreadedComments, FileName, SourceMap};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{error::Error, parse_file_as_module, Syntax, TsConfig};

mod v8_test;

use v8_test::{load_cjs_module, load_esm_module};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut errors: Vec<Error> = vec![];
    let comments = SingleThreadedComments::default();

    let cm = Arc::<SourceMap>::default();
    let fm = cm.new_source_file(FileName::Anon, r#"console.log("hello");"#.to_string());

    // NOTE: the platform must only be initialized once
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let ast = parse_file_as_module(
        &fm,
        Syntax::Typescript(TsConfig {
            tsx: false,
            dts: true,
            decorators: false,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: false,
        }),
        EsVersion::Es2020,
        Some(&comments),
        &mut errors,
    )
    .unwrap(); // TODO: create a wrapper that converts the error to
               // something that implements std::error:Error

    let global = context.global(scope);
    let my_func_key = v8::String::new(scope, "ast").unwrap();
    let v8_obj = serde_v8::to_v8(scope, &ast)?;
    global.set(scope, my_func_key.into(), v8_obj);

    let code = v8::String::new(
        scope,
        "globalThis.sum = 5 + 10; console.log('hello'); globalThis.sum",
    )
    .unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));

    // values set on `globalThis` are available in the global scope
    // on all scripts the run in the same context/isolate.
    let code = v8::String::new(scope, "sum").unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));

    let code = v8::String::new(scope, "JSON.stringify(ast)").unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));

    load_cjs_module();

    load_esm_module();

    Ok(())
}
