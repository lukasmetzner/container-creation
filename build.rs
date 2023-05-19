fn main() {
    cc::Build::new()
        .file("src/namespaces.c")
        .compile("anything");
}