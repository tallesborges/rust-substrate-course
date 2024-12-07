// Disable `std` library and `main` entrypoint, because it's not available in WebAssembly.
// OBS: `std` and `main` are only available when running tests.
#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]

// Override the default panic handler when compilling to WebAssembly.
// Reference: https://doc.rust-lang.org/nomicon/panic-handler.html
// #[cfg(target_arch = "wasm32")]
// #[panic_handler]
// unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
//     core::arch::wasm32::unreachable()
// }

// Makes the external function `env.console_log` available, this line adds the import
// `(import "env" "console_log" (func $name (param i32 i32)))` to the WebAssembly module.
#[cfg(target_arch = "wasm32")] // Only available when compiling to WebAssembly.
pub mod ext {
    #[link(wasm_import_module = "env")] // Add the import to the "env" namespace.
    extern "C" {
        /// Calls an external method which prints the provided string in the console.
        /// Obs: The string must be utf-8 encoded.
        ///
        /// - `data` pointer to the begin of the string in memory.
        /// - `len` string length in bytes, not chars.
        pub fn console_log(data: *const u8, len: u32);

        /// Calls an external method which provides an random float number between 0 and 1.
        pub fn get_random() -> f64;

        // If you want to call more external function, simply add them here.
        // IMPORTANT: Functions declared here must only use primitives types as parameters and return.
        // raw pointers `*const T` will become `i32` in the WebAssembly interface.
        // Example:
        // ```rust
        // pub fn my_method(a: u32, b: u32) -> f64;
        // ```
    }
}

/// Mock the external functions when running tests (otherwise `cargo test` doesn't work).
#[cfg(not(target_arch = "wasm32"))]
pub mod ext {
    /// Print the provided string in the terminal.
    ///
    /// # Safety
    /// This function is unsafe because it uses raw pointers, and it's only used when running tests.
    pub unsafe fn console_log(data: *const u8, len: u32) {
        unsafe {
            let slice = core::slice::from_raw_parts(data, len as usize);
            if let Ok(message) = core::str::from_utf8(slice) {
                println!("{message}");
            }
        }
    }

    /// Calls an external method which provides an random float number between 0 and 1.
    ///
    /// # Safety
    /// Unsafe to match the external counterpart, only used when running tests.
    #[must_use]
    pub unsafe fn get_random() -> f64 {
        rand::random()
    }
}

// The message to be logged to the console.
// When compiled to WebAssembly, this message will be initialized by the `data` section in WebAssembly
// code, example:
// (data $0 (i32.const 1048576) "hello, world!")
// Where `1048576` is the memory address where the message is stored.
static MESSAGE: &[u8] = b"hello, world!\0";

/// Adds two numbers and logs a message to the console.
/// The `no_mangle` guarantees this function will be exported as `add` in the WebAssembly.
#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    unsafe {
        // Call the external function to log the message.
        #[allow(clippy::cast_possible_truncation)]
        ext::console_log(MESSAGE.as_ptr(), MESSAGE.len() as u32);
    }
    a + b
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#[no_mangle]
pub extern "C" fn random() -> i32 {
    unsafe {
        let bytes = f64::to_ne_bytes(ext::get_random());
        i64::from_ne_bytes(bytes) as i32
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
