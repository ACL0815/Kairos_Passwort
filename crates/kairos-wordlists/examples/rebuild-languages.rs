//! Rebuilds English and Latin without network access.
//!
//! Modelled after: EFF <https://www.eff.org/dice> (reviewed word data).

#[path = "../tools/languages.rs"]
mod pipeline;

fn main() -> pipeline::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.is_empty() && args != ["--check"] {
        return Err("usage: rebuild-languages [--check]".into());
    }
    let build = pipeline::build()?;
    print!("{}", build.report);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/languages");
    if args == ["--check"] {
        pipeline::verify_output("english", &build.english)?;
        pipeline::verify_output("latin", &build.latin)?;
        if std::fs::read_to_string(root.join("report.txt"))? != build.report {
            return Err("filter report differs".into());
        }
        if std::fs::read_to_string(root.join("latin-glosses.tsv"))? != build.latin_glosses {
            return Err("Latin source references differ".into());
        }
    } else {
        std::fs::write(root.join("english.txt"), build.english)?;
        std::fs::write(root.join("latin.txt"), build.latin)?;
        std::fs::write(root.join("report.txt"), build.report)?;
        std::fs::write(root.join("latin-glosses.tsv"), build.latin_glosses)?;
    }
    Ok(())
}
