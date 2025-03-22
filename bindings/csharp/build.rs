use std::{env, fs};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    csbindgen::Builder::default()
        .input_extern_file(format!("{crate_dir}/src/engine.rs"))
        .input_extern_file(format!("{crate_dir}/src/result.rs"))
        .input_extern_file(format!("{crate_dir}/src/decision.rs"))
        .input_extern_file(format!("{crate_dir}/src/custom_node.rs"))
        .input_extern_file(format!("{crate_dir}/src/languages/native.rs"))
        .input_extern_file(format!("{crate_dir}/src/expression.rs"))
        .csharp_dll_name("ZenEngine")
        .generate_csharp_file("./ZenEngine.g.cs")
        .unwrap();
}
