fn main() {
    println!("cargo:rustc-link-search=C:/rust/fsr2-rs/FidelityFX-FSR2/bin/ffx_fsr2_api");

    println!("cargo:rustc-link-lib=ffx_fsr2_api_vk_x64");
    println!("cargo:rustc-link-lib=ffx_fsr2_api_x64");

    let bindings = bindgen::Builder::default()
        .header(r"FidelityFX-FSR2\src\ffx-fsr2-api\ffx_fsr2.h")
        .header(r"FidelityFX-FSR2\src\ffx-fsr2-api\vk\ffx_fsr2_vk.h")
        .header(r"FidelityFX-FSR2\src\ffx-fsr2-api\vk\shaders\ffx_fsr2_shaders_vk.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/ffi.rs")
        .expect("Couldn't write bindings!");
}
