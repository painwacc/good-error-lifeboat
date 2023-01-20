use std::{ffi::OsStr, fs, process::Command};

use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
    for e in walkdir::WalkDir::new("..") {
        let e = e?;
        if e.path().extension() != Some(OsStr::new("wacc")) {
            continue;
        }

        let p = e.path();

        let stderr_path = p.with_extension("stderr");

        if std::fs::metadata(&stderr_path).is_ok() {
            continue;
        }

        dbg!(p);

        let cmd_out = Command::new("../refCompile")
            .arg(p)
            .output()
            .with_context(|| format!("Failed to run `../refCompile {p:?}`"))?;

        let stdout = String::from_utf8(cmd_out.stdout)?;
        let (_, err) = stdout
            .split_once("Errors detected during compilation!")
            .ok_or_else(|| anyhow!("No split found for {p:?}"))?;

        fs::write(&stderr_path, &err)?;
    }

    Ok(())
}
