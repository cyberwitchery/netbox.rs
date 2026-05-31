fn main() {
    println!("cargo:rustc-check-cfg=cfg(docsrs)");
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:rustc-cfg=docsrs");
    }
}
