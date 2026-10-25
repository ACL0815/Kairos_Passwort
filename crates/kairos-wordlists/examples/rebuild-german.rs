//! Rebuilds the German list without network access.
//!
//! Modelled after: dys2p/wordlists-de <https://github.com/dys2p/wordlists-de/tree/6ef31b9aefb8735a7b066592393d12843ec502cd>.

#[path = "../tools/pipeline.rs"]
mod pipeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = pipeline::build()?;
    for (stage, count) in build.stages {
        println!("{stage}: {count}");
    }
    let target = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/german.txt");
    if std::env::args().nth(1).as_deref() == Some("--check") {
        if std::fs::read_to_string(target)? != build.text {
            return Err("embedded list differs from pipeline output".into());
        }
    } else {
        std::fs::write(target, build.text)?;
    }
    Ok(())
}
