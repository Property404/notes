use anyhow::Result;
use clap::Parser;
use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
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

fn edit_file(path: impl AsRef<OsStr>) -> Result<()> {
    static EDITOR: OnceLock<(String, Vec<String>)> = OnceLock::new();
    let (editor, args) = &EDITOR.get_or_init(|| {
        let args = env::var("EDITOR").expect("$EDITOR not set");
        let mut args = args.split_whitespace().map(String::from);
        let editor = args.next().expect("$EDITOR is blank");
        let args = args.collect();
        (editor, args)
    });
    Command::new(editor).args(&**args).arg(&path).status()?;
    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// A note to view or edit
    note: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let note_path = notes_dir().join(&cli.note);

    if !note_path.exists() {
        println!("Note '{}' doesn't exist.", cli.note);
        print!("Would you like to create it(y/n)?");
        io::stdout().flush()?;

        let mut option = String::new();
        loop {
            io::stdin()
                .read_line(&mut option)
                .expect("Failed to read from stdin");
            let option = option.to_lowercase();
            let option = option.trim();

            if option.starts_with('y') {
                break;
            } else if option.starts_with('n') {
                return Ok(());
            }
        }
    }

    edit_file(&note_path)?;

    Ok(())
}
