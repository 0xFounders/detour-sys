// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/DetourAlloc.cpp");
    println!("cargo:rerun-if-changed=src/DetourAssert.cpp");
    println!("cargo:rerun-if-changed=src/DetourCommon.cpp");
    println!("cargo:rerun-if-changed=src/DetourNavMesh.cpp");
    println!("cargo:rerun-if-changed=src/DetourNavMeshBuilder.cpp");
    println!("cargo:rerun-if-changed=src/DetourNavMeshQuery.cpp");
    println!("cargo:rerun-if-changed=src/DetourNode.cpp");
    println!("cargo:rerun-if-changed=src/extern.cpp");

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .cpp(true)
        .file("src/DetourAlloc.cpp")
        .file("src/DetourAssert.cpp")
        .file("src/DetourCommon.cpp")
        .file("src/DetourNavMesh.cpp")
        .file("src/DetourNavMeshBuilder.cpp")
        .file("src/DetourNavMeshQuery.cpp")
        .file("src/DetourNode.cpp")
        .file("src/extern.cpp")
        .define("DT_POLYREF64", Some("1"))
        .compile("detour");
}
