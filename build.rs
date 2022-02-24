// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/vendor/DetourAlloc.cpp");
    println!("cargo:rerun-if-changed=src/vendor/DetourAssert.cpp");
    println!("cargo:rerun-if-changed=src/vendor/DetourCommon.cpp");
    println!("cargo:rerun-if-changed=src/vendor/DetourNavMesh.cpp");
    println!("cargo:rerun-if-changed=src/vendor/DetourNavMeshBuilder.cpp");
    println!("cargo:rerun-if-changed=src/vendor/DetourNavMeshQuery.cpp");
    println!("cargo:rerun-if-changed=src/vendor/DetourNode.cpp");
    println!("cargo:rerun-if-changed=src/extern.cpp");

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .cpp(true)
        .define("DT_POLYREF64", "1")
        .file("src/vendor/DetourAlloc.cpp")
        .file("src/vendor/DetourAssert.cpp")
        .file("src/vendor/DetourCommon.cpp")
        .file("src/vendor/DetourNavMesh.cpp")
        .file("src/vendor/DetourNavMeshBuilder.cpp")
        .file("src/vendor/DetourNavMeshQuery.cpp")
        .file("src/vendor/DetourNode.cpp")
        .file("src/extern.cpp")
        .compile("detour");
}
