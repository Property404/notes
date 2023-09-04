use anyhow::Result;
use clap::Parser;
use regex::Regex;
use std::{
    cmp::Ordering,
    env,
    ffi::OsStr,
    fs,
    io::{self, Write},
    os::unix::fs::MetadataExt,
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
        let args = env::var("EDITOR")
            .or_else(|_err| env::var("VISUAL"))
            .unwrap_or_else(|_err| String::from("vi"));
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
    note: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let notes_dir = notes_dir();

    if let Some(note) = &cli.note {
        let note_path = notes_dir.join(note);

        if !note_path.exists() {
            println!("Note '{note}' doesn't exist.");
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
    } else {
        let notes: io::Result<Vec<_>> = fs::read_dir(notes_dir)?.collect();
        let mut notes = notes?;

        // Sort by access time
        notes.sort_unstable_by(|a, b| {
            a.metadata()
                .map(|m| m.atime())
                .unwrap_or_default()
                .partial_cmp(&b.metadata().map(|m| m.atime()).unwrap_or_default())
                .unwrap_or(Ordering::Equal)
        });

        if notes.is_empty() {
            println!("Could not find any notes");
            return Ok(());
        }

        println!("Found the following:");
        println!("====================");
        for (index, note) in notes.iter().enumerate() {
            let note = note.file_name();
            let note = note.to_str().expect("Encountered bad unicode in file name");
            println!("{index}. {note}");
        }

        loop {
            print!("Select> ");

            let mut option = String::new();
            io::stdout().flush()?;
            io::stdin().read_line(&mut option)?;
            let option = option.trim();
            let option_lowercase = option.to_lowercase();

            if option_lowercase == "q" || option_lowercase == "quit" {
                break;
            }

            if let Some(regex) = option.strip_prefix('/') {
                let regex = Regex::new(regex)?;
                let matches: Vec<_> = notes
                    .iter()
                    .filter(|v| {
                        regex.is_match(v.file_name().to_str().expect("Bad unicode in file name"))
                    })
                    .collect();
                if matches.is_empty() {
                    println!("No match for '{regex}'");
                } else if matches.len() > 1 {
                    println!("Multiple matches for '{regex}'");
                } else {
                    edit_file(matches[0].path())?;
                    break;
                }
            }

            if let Ok(number) = option.parse::<usize>() {
                if number < notes.len() {
                    edit_file(notes[number].path())?;
                    break;
                }
            }
        }
    }

    Ok(())
}
