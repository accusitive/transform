use rusty_v8 as v8;
pub(crate) fn test_js(s: String){
    
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope =  &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, &s).unwrap();
    let mut script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    println!("Result => {}", result.to_string(scope).unwrap().to_rust_string_lossy(scope));
}