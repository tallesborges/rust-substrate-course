// use std::env;

fn main() {
    if matches!(std::env::var("CARGO_CFG_TARGET_FAMILY").ok().as_deref(), Some("wasm")) {
        // println!("cargo:rustc-link-arg=--import-memory\x1f--max-memory=65536");
        // println!("cargo:rustc-link-arg=-Clink-arg=--initial-memory=65536");
        std::env::set_var("CARGO_ENCODED_RUSTFLAGS", "-Clink-arg=-zstack-size=65536\x1f-Clink-arg=--import-memory\x1f-Ctarget-cpu=mvp");
        // println!("cargo:rustc-link-arg=--max-memory=65536");
    }
    // if matches!(std::env::var("CARGO_CFG_TARGET_FAMILY").ok().as_deref(), Some("wasm")) {
    //     println!("cargo:rustc-link-arg=-zstack-size=65536\x1f-Clink-arg=--import-memory\x1f-Ctarget-cpu=mvp");
    // }
    // println!("cargo:rustc-link-arg=-zstack-size=65536\x1f-Clink-arg=--import-memory\x1f-Ctarget-cpu=mvp");
}

// CARGO_ENCODED_RUSTFLAGS='-Clink-arg=-zstack-size=65536\x1f-Clink-arg=--import-memory' cargo build --release --target=wasm32-unknown-unknown
// CARGO_ENCODED_RUSTFLAGS='-Clink-arg=--import-memory' cargo build --release --target=wasm32-unknown-unknown
// CARGO_ENCODED_RUSTFLAGS="$(printf "-Clink-arg=-zstack-size=65536\x1f-Clink-arg=--import-memory\x1f-Ctarget-cpu=mvp")" cargo build --release --target=wasm32-unknown-unknown