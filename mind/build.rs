// YAI/Api/build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Rileva la root del progetto per trovare il Kernel
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let kernel_include_path = PathBuf::from(&manifest_dir)
        .parent()
        .unwrap()
        .join("Kernel")
        .join("include");

    // 2. Istruisci Cargo a cercare gli header del Kernel
    println!("cargo:rustc-link-search=native={}", kernel_include_path.display());
    
    // 3. Linka le librerie di sistema necessarie per SHM e POSIX (rt = real time)
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=rt");
    }

    // 4. Re-run se cambiano i file del Kernel (opzionale ma consigliato)
    println!("cargo:rerun-if-changed={}", kernel_include_path.display());

    // Nota: Se hai file .c specifici nel Bridge che devono essere compilati con Rust,
    // usiamo il crate 'cc' qui:
    /*
    cc::Build::new()
        .file("src/bridge/native_helper.c")
        .include(kernel_include_path)
        .compile("yaibridge");
    */
}
