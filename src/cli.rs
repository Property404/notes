use clap::{Parser, ValueEnum, ValueHint};

/// The sorting method used when retrieving a list of notes.
#[derive(PartialEq, Copy, Clone, ValueEnum)]
pub enum SortBy {
    /// Sort by access time, oldest access first.
    AccessTime,
    /// Sort alphabetically, from 'A' to 'Z'.
    Alphabetical,
    /// Do not sort.
    None,
}

#[derive(Parser)]
#[command(author, about, long_about = None, version)]
pub struct Cli {
    /// Execute git command
    #[clap(long, allow_hyphen_values=true, num_args = 1..,value_name="ARGS", group="main")]
    pub git: Option<Vec<String>>,

    /// Execute ripgrep command
    #[clap(long, allow_hyphen_values=true, num_args = 1..,value_name="ARGS", group="main")]
    pub rg: Option<Vec<String>>,

    /// Execute a command in the notes directory. `{}` will be expanded to all note files as a
    /// separate arg
    #[clap(long, allow_hyphen_values=true, num_args = 1..,value_name="ARGS", group="main")]
    pub exec: Option<Vec<String>>,

    /// Get directory of note
    #[clap(long, value_name = "NOTE", group = "main")]
    pub dir: Option<Option<String>>,

    /// List notes
    #[clap(long, group = "main")]
    pub list: bool,

    /// Set sorting method
    #[clap(long)]
    pub sort_by: Option<SortBy>,

    /// Remove note
    #[clap(long, value_name = "NOTE", alias = "rm", group = "main")]
    pub remove: Option<String>,

    /// A note to view or edit
    #[clap(value_hint = ValueHint::Unknown, group = "main")]
    pub note: Option<String>,
}
