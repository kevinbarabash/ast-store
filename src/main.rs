use std::error;

mod ast;
mod loader;
mod parser;

use loader::load_esm_module;

fn main() -> Result<(), Box<dyn error::Error>> {
    // NOTE: the platform must only be initialized once
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

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

    load_esm_module();

    let code = "console.log(\"hello, \");\ndebugger;\nconsole.log(\"world!\");";
    let ast = parser::parse(code.to_string());

    // TODO: figure out how to call JS functions from Rust so that
    // we don't have to set globals like this.
    let global = context.global(scope);
    let my_func_key = v8::String::new(scope, "ast").unwrap();
    let v8_obj = serde_v8::to_v8(scope, &ast)?;
    global.set(scope, my_func_key.into(), v8_obj);

    let code = v8::String::new(scope, "JSON.stringify(ast, null, 2)").unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));

    Ok(())
}
