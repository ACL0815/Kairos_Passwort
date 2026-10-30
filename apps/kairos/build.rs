fn main() {
    println!("cargo:rerun-if-changed=assets/kairos.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        winresource::WindowsResource::new()
            .set_icon("assets/kairos.ico")
            .set("ProductName", "Kairos Passwort")
            .set("FileDescription", "Kairos Passwort")
            .compile()
            .expect("Kairos icon resource could not be compiled");
    }
}
