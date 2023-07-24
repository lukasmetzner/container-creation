fn main() {
    println!("cargo:rerun-if-changed=src/namespaces.c");
    cc::Build::new()
        .file("src/namespaces.c")
        .compile("anything");
}