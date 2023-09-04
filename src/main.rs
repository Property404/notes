use clap::Parser;
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

fn notes_dir() -> &'static Path {
    static NOTES_DIR: OnceLock<PathBuf> = OnceLock::new();
    NOTES_DIR
        .get_or_init(|| {
            let path = Path::new(&env::var("HOME").expect("$HOME not set")).join(".config/notes");
            fs::create_dir_all(&path).expect("Could not create notes directory!");
            path
        })
        .as_ref()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// A note to view or edit
    note: String,
}

fn main() {
    let cli = Cli::parse();

    let note_path = notes_dir().join(cli.note);

    println!("{note_path:?}");
    if note_path.exists() {
        println!("Exists!");
    }
}
