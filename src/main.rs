use std::error;
use std::sync::Arc;
use swc_common::{comments::SingleThreadedComments, FileName, SourceMap};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{error::Error, parse_file_as_module, Syntax, TsConfig};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut errors: Vec<Error> = vec![];
    let comments = SingleThreadedComments::default();

    let cm = Arc::<SourceMap>::default();
    let fm = cm.new_source_file(FileName::Anon, r#"console.log("hello");"#.to_string());

    let module = parse_file_as_module(
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

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let v8_obj = serde_v8::to_v8(scope, &module)?;

    let code = v8::String::new(
        scope,
        "import 'fs'; 'Hello' + ' World!'; console.log('hello')",
    )
    .unwrap();

    let resource_name = v8::String::new(scope, "<resource>").unwrap();
    let source_map_url = v8::undefined(scope);
    let script_origin = v8::ScriptOrigin::new(
        scope,
        resource_name.into(),
        0,
        0,
        false,
        0,
        source_map_url.into(),
        false,
        false,
        true, // is module
    );

    let src = v8::script_compiler::Source::new(code, Some(&script_origin));
    let module = v8::script_compiler::compile_module(scope, src).unwrap();

    eprintln!("finished compiling");

    module
        .instantiate_module(scope, module_resolve_callback)
        .unwrap();
    module.evaluate(scope).unwrap();

    eprintln!("finished evaluating");

    let global = context.global(scope);

    let my_func_key = v8::String::new(scope, "ast").unwrap();
    global.set(scope, my_func_key.into(), v8_obj);
    let code = v8::String::new(scope, "console.log('hello'); JSON.stringify(ast)").unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));

    Ok(())
}

fn module_resolve_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    _specifier: v8::Local<'a, v8::String>,
    _import_assertions: v8::Local<'a, v8::FixedArray>,
    _referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let scope = &mut unsafe { v8::CallbackScope::new(context) };

    eprintln!("_specifier = {:#}", _specifier.to_rust_string_lossy(scope));

    let resource_name = v8::String::new(scope, "<resource>").unwrap();
    let source_map_url = v8::undefined(scope);

    let origin = v8::ScriptOrigin::new(
        scope,
        resource_name.into(),  // resource_name
        0,                     // resource_line_offset
        0,                     // resource_line_column_offset
        false,                 // resource_is_shared_cross_origin
        0,                     // script_id
        source_map_url.into(), // source_map_url
        false,                 // resource_is_opaque
        false,                 // is_wasm
        true,                  // is_module
    );

    // let origin = mock_script_origin(scope, "module.js");
    let src = v8::String::new(scope, "export const a = 'a';").unwrap();
    let source = v8::script_compiler::Source::new(src, Some(&origin));
    let module = v8::script_compiler::compile_module(scope, source).unwrap();
    Some(module)
}
