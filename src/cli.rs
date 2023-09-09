use clap::{Parser, ValueEnum, ValueHint};

#[derive(PartialEq, Copy, Clone, ValueEnum)]
pub enum SortBy {
    LastAccess,
    Alphabetical,
    None,
}

#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
    /// Execute git command
    #[clap(long, allow_hyphen_values=true, num_args = 1..,value_name="ARGS")]
    pub git: Option<Vec<String>>,

    /// Execute ripgrep command
    #[clap(long, allow_hyphen_values=true, num_args = 1..,value_name="ARGS")]
    pub rg: Option<Vec<String>>,

    /// Execute a command in the notes directory
    #[clap(long, allow_hyphen_values=true, num_args = 1..,value_name="ARGS")]
    pub exec: Option<Vec<String>>,

    /// Get directory of note
    #[clap(long, value_name = "NOTE")]
    pub dir: Option<Option<String>>,

    /// List notes
    #[clap(long)]
    pub list: bool,

    /// Set sorting method
    #[clap(long)]
    pub sort_by: Option<SortBy>,

    /// A note to view or edit
    #[clap(long, value_hint = ValueHint::Unknown)]
    pub note: Option<String>,
}
