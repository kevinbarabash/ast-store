use std::fs;
use std::path::Path;

struct Loader {
    cwds: Vec<String>,
    // TODO: add a hashmap to lookup modules by path so that we only
    // have to process each module once.
}

pub fn load_esm_module() -> Option<()> {
    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let loader = Loader {
        cwds: vec![".".to_string()],
    };
    scope.set_slot(loader);

    let module = load_module("./js/index.js", scope)?;

    module.evaluate(scope).unwrap();

    let ns = module.get_module_namespace();
    eprintln!("ns.is_object() = {}", ns.is_object());
    let obj = ns.to_object(scope)?;
    let key = v8::String::new(scope, "foobar")?;
    let foobar = obj.get(scope, key.into()).unwrap();
    eprintln!("foobar = {}", foobar.to_rust_string_lossy(scope));

    Some(())
}

fn load_module<'a>(
    path: &str,
    scope: &mut v8::HandleScope<'a>,
) -> Option<v8::Local<'a, v8::Module>> {
    let loader = scope.get_slot_mut::<Loader>().unwrap();

    let current_cwd = loader.cwds.last().unwrap().clone();
    let parent_dir = Path::new(&path).parent().unwrap();

    loader.cwds.push(
        Path::new(&current_cwd)
            .join(parent_dir)
            .to_str()
            .unwrap()
            .to_string(),
    );

    let path = Path::new(&current_cwd).join(path);
    let code = match fs::read_to_string(&path) {
        Ok(src) => v8::String::new(scope, &src).unwrap(),
        Err(e) => {
            eprintln!("error reading file {:#?}: {:#?}", path, e);
            return None;
        }
    };

    let resource_name = v8::String::new(scope, path.to_str().unwrap()).unwrap();
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

    module
        .instantiate_module(scope, module_resolve_callback)
        .unwrap();

    let loader = scope.get_slot_mut::<Loader>().unwrap();
    loader.cwds.pop();

    Some(module)
}

fn module_resolve_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    _import_assertions: v8::Local<'a, v8::FixedArray>,
    _referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let scope = &mut unsafe { v8::CallbackScope::new(context) };
    let path = specifier.to_rust_string_lossy(scope);

    load_module(&path, scope)
}
