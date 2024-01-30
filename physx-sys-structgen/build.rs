use std::path::Path;

fn main() {
    let physx_dir= std::env::var("PHYSX_DIR").expect("Expected PHYSX_DIR absolute path to physx-rs/physx-sys");
    let physx_include_dir = Path::new(&physx_dir).join("physx/physx/include");
    let structgen_dir = Path::new(&physx_dir).join("src/structgen");
    let structgen_path = structgen_dir.join("structgen.cpp");

    cc::Build::new()
        .cpp(true)
        .define("NDEBUG", None)
        .define("PX_PHYSX_STATIC_LIB", None)
        .include(physx_include_dir)
        .file(structgen_path)
        .extra_warnings(false)
        .std("c++14")
        .static_flag(true)
        .compile("libstructgen.a");
}