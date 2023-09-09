use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

const BASH_COMPLETIONS_DIR_VAR: &str = "BASH_COMPLETIONS_DIR";
const OUT_FILE_NAME: &str = "dev_dagans_notes_completion.bash";
const FOOTER_FILE_PATH: &str = "scripts/bash_completion_footer.bash";

include!("src/cli.rs");

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={FOOTER_FILE_PATH}");
    println!("cargo:rerun-if-env-changed={BASH_COMPLETIONS_DIR_VAR}");

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_file = root.join("generated").join(OUT_FILE_NAME);

    // Construct original clap_complete script
    let mut cmd = Cli::command();
    let mut script = BufWriter::new(Vec::new());
    let name = cmd.get_name().to_string();
    generate(Shell::Bash, &mut cmd, &name, &mut script);
    let script = String::from_utf8(script.into_inner()?)?;

    // Strip invocation, because we will write our own
    let mut script: Vec<_> = script.lines().collect();
    assert!(script
        .last()
        .expect("Bug: Script empty")
        .starts_with("complete"));
    script.pop();
    script.push("");
    let mut script = script.join("\n");

    // Add footer
    let footer = fs::read_to_string(root.join(FOOTER_FILE_PATH))?;
    script.push_str(&footer);

    // Finish up
    File::create(&out_file)?.write_all(script.as_bytes())?;
    if let Some(completions_dir) = env::var_os(BASH_COMPLETIONS_DIR_VAR) {
        fs::copy(out_file, PathBuf::from(completions_dir).join(OUT_FILE_NAME))?;
    };

    Ok(())
}
