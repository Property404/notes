use anyhow::{anyhow, bail, Result};
use clap::Parser;
use notes::{
    cli::{Cli, SortBy},
    config::Config,
    manifest::Manifest,
};
use regex::Regex;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::{
    env,
    fs::{self, Metadata},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    sync::OnceLock,
};
use walkdir::{DirEntry, WalkDir};

struct Note {
    name: String,
    path: PathBuf,
}

impl Note {
    fn new(name: impl Into<String>) -> Note {
        let name = name.into();
        Note {
            path: notes_dir().join(&name).with_extension("md"),
            name,
        }
    }

    fn from_path(path: PathBuf) -> Result<Note> {
        let name = path
            .file_name()
            .ok_or_else(|| anyhow!("No file name in path!"))?
            .to_str()
            .ok_or_else(|| anyhow!("File name not valid UTF-8!"))?;
        let Some(name) = name.strip_suffix(".md") else {
            bail!("Uh, yikes, this doesn't look like a note file")
        };
        Ok(Note {
            name: name.into(),
            path,
        })
    }

    fn metadata(&self) -> Result<Metadata> {
        Ok(self.path.metadata()?)
    }
}

fn all_notes(sort_by: SortBy) -> Result<Vec<Note>> {
    let mut notes = Vec::new();
    let matcher = |entry: &DirEntry| {
        let file_name = entry.file_name().to_str();
        let is_hidden = file_name.map(|s| s.starts_with('.')).unwrap_or(false);
        !is_hidden
    };

    let walker = WalkDir::new(notes_dir()).into_iter();
    for e in walker.filter_entry(matcher).filter_map(|e| e.ok()) {
        let path = e.path();
        if let Some(path_name) = path.to_str() {
            if path_name.ends_with(".md") {
                notes.push(Note::from_path(path.into())?)
            }
        }
    }

    // Sort by access time
    match sort_by {
        SortBy::None => {}
        SortBy::Alphabetical => notes.sort_unstable_by(|a, b| a.name.cmp(&b.name)),
        SortBy::AccessTime => notes.sort_unstable_by(|a, b| {
            a.metadata()
                .map(|m| m.atime())
                .unwrap_or_default()
                .cmp(&b.metadata().map(|m| m.atime()).unwrap_or_default())
        }),
    };

    Ok(notes)
}

fn init_notes_repo(path: &Path) -> Result<()> {
    // Create git repo
    fs::create_dir_all(path)?;
    env::set_current_dir(path)?;
    Command::new("git").arg("init").output()?;

    // Create .gitignore
    let gitignore = ["*", "!notes", "!*.md", "!.gitignore", ""];
    fs::write(".gitignore", gitignore.join("\n"))?;

    // Create manifest
    fs::write("manifest.toml", toml::to_string(&Manifest::new()?)?)?;

    // First commit
    Command::new("git").args(["add", "."]).output()?;
    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .output()?;

    Ok(())
}

fn notes_dir() -> &'static Path {
    static NOTES_DIR: OnceLock<PathBuf> = OnceLock::new();
    NOTES_DIR
        .get_or_init(|| {
            let config = Config::load().expect("Failed to load config");
            let path = config.repo_path;

            if !path.join(".git").exists() {
                init_notes_repo(&path)
                    .unwrap_or_else(|e| panic!("Could not create notes repo: {e}"));
            }

            let path = path.join("notes");
            fs::create_dir_all(&path).expect("Could not create notes directory!");
            env::set_current_dir(&path).expect("Failed to change directory");
            path
        })
        .as_ref()
}

fn edit_note(note: &Note) -> Result<()> {
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

    let previously_existed = note.path.exists();

    Command::new(editor).args(args).arg(&note.path).status()?;

    if note.path.exists() {
        Command::new("git").arg("add").arg(&note.path).output()?;
        Command::new("git")
            .args(["commit", "-m"])
            .arg(format!(
                "{}] {}",
                if previously_existed {
                    "edited"
                } else {
                    "created"
                },
                &note.name
            ))
            .output()?;
    }
    Ok(())
}

fn remove_note(note: &Note) -> Result<()> {
    if !note.path.exists() {
        bail!("No such note: {}", note.name);
    }
    Command::new("git").arg("rm").arg(&note.path).output()?;
    Command::new("git")
        .args(["commit", "-m", &format!("removed] {}", note.name)])
        .output()?;
    Ok(())
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    if let Some(note) = &cli.note {
        let note = Note::new(note);

        if !note.path.exists() {
            println!("Note '{}' doesn't exist.", &note.name);

            let mut rleditor = DefaultEditor::new()?;
            loop {
                let option = match rleditor.readline("Would you like to create it(y/n)?") {
                    Ok(option) => option,
                    Err(ReadlineError::Interrupted) => {
                        return Ok(ExitCode::FAILURE);
                    }
                    Err(err) => {
                        return Err(err.into());
                    }
                }
                .to_lowercase();
                let option = option.trim();

                if option.starts_with('y') {
                    break;
                } else if option.starts_with('n') {
                    return Ok(ExitCode::SUCCESS);
                }
            }
        }

        edit_note(&note)?;
    } else if let Some(args) = &cli.git {
        std::env::set_current_dir(notes_dir())?;
        Command::new("git").args(args).status()?;
    } else if let Some(args) = &cli.rg {
        std::env::set_current_dir(notes_dir())?;
        Command::new("rg").args(args).status()?;
    } else if let Some(args) = &cli.exec {
        std::env::set_current_dir(notes_dir())?;
        let mut args = args.iter();
        let command = args
            .next()
            .expect("Bug: No args(should have been caught by Clap)");

        let mut expanded_args = Vec::new();
        let sort_by = cli.sort_by.unwrap_or(SortBy::None);
        let notes: Vec<String> = all_notes(sort_by)?
            .into_iter()
            .map(|note| format!("{}.md", note.name))
            .collect();

        for arg in args {
            if arg == "{}" {
                expanded_args.extend(&notes);
            } else {
                expanded_args.push(arg);
            }
        }

        Command::new(command).args(&expanded_args).status()?;
    } else if let Some(note) = &cli.dir {
        if let Some(note) = note {
            let note = Note::new(note);
            if !note.path.exists() {
                bail!("No note named '{}'", note.name);
            }
            println!("{}", note.path.display());
        } else {
            println!("{}", notes_dir().display());
        }
    } else if cli.list {
        for note in all_notes(cli.sort_by.unwrap_or(SortBy::Alphabetical))? {
            println!("{}", note.name);
        }
    } else if let Some(note) = cli.remove {
        let note = Note::new(note);
        remove_note(&note)?;
    } else {
        let notes = all_notes(cli.sort_by.unwrap_or(SortBy::AccessTime))?;

        if notes.is_empty() {
            println!("Could not find any notes");
            return Ok(ExitCode::SUCCESS);
        }

        println!("Found the following:");
        println!("====================");
        for (index, note) in notes.iter().enumerate() {
            let note = &note.name;
            println!("{index}. {note}");
        }

        let mut rleditor = DefaultEditor::new()?;
        loop {
            let option = match rleditor.readline("Select> ") {
                Ok(option) => option,
                Err(ReadlineError::Interrupted) => {
                    return Ok(ExitCode::FAILURE);
                }
                Err(err) => {
                    return Err(err.into());
                }
            };
            let option = option.trim();

            let option_lowercase = option.to_lowercase();

            if option_lowercase == "q" || option_lowercase == "quit" {
                break;
            }

            if option_lowercase == "l" || option_lowercase == "last" {
                edit_note(&notes[notes.len() - 1])?;
                break;
            }

            if let Some(regex) = option.strip_prefix('/') {
                let regex = Regex::new(regex)?;
                let matches: Vec<_> = notes.iter().filter(|v| regex.is_match(&v.name)).collect();
                if matches.is_empty() {
                    println!("No match for '{regex}'");
                } else if matches.len() > 1 {
                    println!("Multiple matches for '{regex}'");
                } else {
                    edit_note(matches[0])?;
                    break;
                }
            }

            if let Ok(number) = option.parse::<usize>() {
                if number < notes.len() {
                    edit_note(&notes[number])?;
                    break;
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
