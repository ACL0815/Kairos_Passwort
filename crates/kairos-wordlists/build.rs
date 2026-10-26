//! Build-only masking of public dictionary data; this is not encryption.
//!
//! Modelled after: CasualX/obfstr (separate stored bytes from runtime plaintext)
//! <https://github.com/CasualX/obfstr/blob/9e56c1938dfd3bd65fe305fbafcb57394c701356/src/bytes.rs>.
//! No implementation code copied. See README.md for limits and data licenses.

use std::{env, error::Error, fs, path::PathBuf};

use sha2::{Digest, Sha256};

fn main() -> Result<(), Box<dyn Error>> {
    let root =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or("missing manifest directory")?);
    let out = PathBuf::from(env::var_os("OUT_DIR").ok_or("missing output directory")?);
    let mut declarations = String::new();
    for (name, relative) in [
        ("GERMAN", "data/german"),
        ("ENGLISH", "data/languages/english"),
        ("LATIN", "data/languages/latin"),
    ] {
        println!("cargo:rerun-if-changed={relative}.txt");
        println!("cargo:rerun-if-changed={relative}.sha256");
        let text = fs::read(root.join(format!("{relative}.txt")))?;
        let digest = Sha256::digest(&text);
        // Bind to the reviewed plaintext, not merely whichever input is present.
        let approved = fs::read_to_string(root.join(format!("{relative}.sha256")))?;
        if format!("{digest:x}") != approved.trim() {
            return Err(format!("embedded word list {name}: approved SHA-256 mismatch").into());
        }
        let mut mask = vec![0; text.len()];
        getrandom::fill(&mut mask)
            .map_err(|error| format!("word list mask RNG failed: {error}"))?;
        let encoded: Vec<_> = text
            .iter()
            .zip(&mask)
            .map(|(byte, key)| byte ^ key)
            .collect();
        fs::write(out.join(format!("{name}.masked")), encoded)?;
        fs::write(out.join(format!("{name}.mask")), mask)?;
        declarations.push_str(&format!(
            "static {name}: EmbeddedData = EmbeddedData {{\n\
             encoded: include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{name}.masked\")),\n\
             mask: include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{name}.mask\")),\n\
             sha256: {digest:?}, text: OnceLock::new(),\n}};\n"
        ));
    }
    fs::write(out.join("embedded.rs"), declarations)?;
    Ok(())
}
