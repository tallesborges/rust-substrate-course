use wasmtime::{
    Caller, Config, Engine, Func, Instance, Memory, MemoryType, Module, OptLevel, Store,
};

const WAT_CODE: &[u8] = include_bytes!("../../wasm_runtime.wat");

struct MyState {}

fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.cranelift_opt_level(OptLevel::SpeedAndSize);
    config
        .cranelift_nan_canonicalization(false)
        .wasm_tail_call(false)
        .parallel_compilation(true)
        .wasm_multi_value(false)
        .wasm_multi_memory(false)
        .wasm_bulk_memory(true)
        .wasm_relaxed_simd(false)
        .wasm_simd(false);

    let engine = Engine::new(&config)?;
    let module = Module::new(&engine, WAT_CODE)?;
    let mut store = Store::new(&engine, MyState {});

    let memory_type = MemoryType::new(2, None);
    let memory = Memory::new(&mut store, memory_type)?;

    let console_log_func =
        Func::wrap(&mut store, move |caller: Caller<'_, MyState>, ptr: i32, len: i32| {
            println!("console_log called, {}, {}", ptr, len);

            let data = memory.data(&caller)[ptr as usize..(ptr + len) as usize].to_vec();
            let string = String::from_utf8(data).unwrap();
            println!("From memory: {}", string);

            Ok(())
        });

    let imports = [memory.into(), console_log_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    let add = instance.get_typed_func::<(u32, u32), u32>(&mut store, "add")?;
    let mul = instance.get_typed_func::<(u32, u32), u32>(&mut store, "mul")?;
    let hello_world = instance.get_typed_func::<(), ()>(&mut store, "hello_world")?;

    println!("add = {}", add.call(&mut store, (5, 2))?);
    println!("mul = {}", mul.call(&mut store, (5, 2))?);
    hello_world.call(&mut store, ())?;

    Ok(())
}
