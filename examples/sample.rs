use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "sample-wasm/target/wasm32-unknown-unknown/debug/sample-wasm.wasm")?;

    // Create a `Linker` and define our host function in it:
    let mut linker = Linker::new(&engine);
    linker.func_wrap("host", "hello", |caller: Caller<'_, u32>, param: i32| {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
    })?;

    // Use the `linker` to instantiate the module, which will automatically
    // resolve the imports of the module using name-based resolution.
    let mut store = Store::new(&engine, 0);
    let instance = linker.instantiate(&mut store, &module)?;
    let main= instance.get_typed_func::<(), (), _>(&mut store, "main")?;
    main.call(&mut store, ())?;

    Ok(())
}