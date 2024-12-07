use wasmtime::{
    Caller, Config, Engine, Func, Instance, Memory, MemoryType, Module, OptLevel, Store,
};

// Código WebAssembly em formato de texto.
const WAT_CODE: &[u8] = include_bytes!("hello.wat");

struct State {
    name: String,
    count: usize,
}

fn main() -> anyhow::Result<()> {
    // Configura o compilador WebAssembly.
    let mut config = Config::new();
    // Otimize para velocidade e tamanho.
    config.cranelift_opt_level(OptLevel::SpeedAndSize);
    // Desativa algumas funcionalidades que opicionais do WebAssembly.
    config.cranelift_nan_canonicalization(false);
    config.wasm_tail_call(false);
    config.parallel_compilation(true);
    config.wasm_multi_value(false);
    config.wasm_multi_memory(false);
    config.wasm_bulk_memory(true);
    // config.wasm_reference_types(false);
    // config.wasm_threads(false);
    config.wasm_relaxed_simd(false);
    config.wasm_simd(false);

    // Configura a Engine com as opções definidas.
    let engine = Engine::new(&config)?;

    // Compila o código WebAssembly.
    let module = Module::new(&engine, WAT_CODE)?;

    // Inicia um Store com o estado inicial.
    let mut store = Store::new(
        &engine,
        State {
            name: "Wasm".to_string(),
            count: 0,
        },
    );

    // Chamar a função
    // let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, State>| {
    //     println!("Calling back...");
    //     println!("> {}", caller.data().name);
    //     caller.data_mut().count += 1;
    // });

    // let memory_type = MemoryType::new(1, Some(16));
    // let memory = Memory::new(&mut store, memory_type)?;

    // let imports = [memory.into(), hello_func.into()];
    let instance = Instance::new(&mut store, &module, &[])?;

    let run = instance.get_typed_func::<(u32, u32), u32>(&mut store, "add")?;
    let result = run.call(&mut store, (10, 17))?;

    println!("{}", result);
    Ok(())
}
