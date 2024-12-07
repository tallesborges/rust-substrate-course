// Disable `std` library and `main` entrypoint, because they are not available in WebAssembly.
// OBS: `std` and `main` are only available when running tests.
#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]

// Override the default panic handler when compilling to WebAssembly.
// Reference: https://doc.rust-lang.org/nomicon/panic-handler.html
#[cfg(target_arch = "wasm32")]
#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

/// Adds two numbers.
#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
pub extern "C" fn mul(a: u32, b: u32) -> u32 {
    a * b
}

#[no_mangle]
pub extern "C" fn hello_world() {
    let str = "Hello World, by talles!";

    unsafe { ext::console_log(str.as_ptr() as u32, str.len() as u32) }
}

// #[no_mangle]
// pub const extern "C" fn hello_world() -> *const u8 {
//     b"Hello World".as_ptr()
// }

// Makes the external function `env.console_log` available, this line adds the import
// `(import "env" "console_log" (func $name (param i32 i32)))` to the WebAssembly module.
#[cfg(target_arch = "wasm32")] // Only available when compiling to WebAssembly.
pub mod ext {
    #[link(wasm_import_module = "env")] // Add the import to the "env" namespace.
    extern "C" {
        pub fn console_log(data: u32, len: u32);
        pub fn hello_world();
    }
}

/// Mock the external functions when running tests (otherwise `cargo test` doesn't work).
#[cfg(not(target_arch = "wasm32"))]
pub mod ext {
    pub unsafe fn console_log(offset: u32, len: u32) {
        unsafe {
            let ptr = offset as *const u8;
            let slice = core::slice::from_raw_parts(ptr, len as usize);
            if let Ok(message) = core::str::from_utf8(slice) {
                println!("{message}");
            }
        }
    }

    pub unsafe fn hello_world() {
        println!("Hello World");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
