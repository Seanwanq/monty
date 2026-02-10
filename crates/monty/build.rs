fn main() {
    csbindgen::Builder::new()
        .input_extern_file("src/ffi.rs")
        .csharp_class_name("MontyNative")
        .csharp_dll_name("monty")
        .csharp_namespace("Kudoo.Infrastructure.Native")
        .generate_csharp_file("../../../../src/Kudoo.Infrastructure/Native/MontyNative.g.cs")
        .unwrap();
}
