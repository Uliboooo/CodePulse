use chrono::Local;
use clap::{Args, Parser, Subcommand, command};
use local_issues_lib::{self};
use std::{env, fmt::Display, io};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
enum Error {
    LocalIssue(local_issues_lib::Error),
    Io(io::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LocalIssue(error) => write!(f, "{:?}", error),
            Error::Io(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Debug, Parser)]
#[command(propagate_version = true)]
#[command(name = "cpls", version = VERSION)]
struct Cli {
    #[command(subcommand)]
    subcommands: Commands,

    #[arg(short = 'y', long = "yes")]
    non_interactive: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// initialize project based on arg path or current dir(default)
    Init(Init),
    /// manage issues, e.g. create, list and remove...
    Issue(Issue),
    /// control about comment, e.g. add, hide comment, etc...
    Comment(Comment),
    /// open issue
    Open(Open),
}

#[derive(Debug, Args)]
struct Init {
    title: Option<String>,

    #[arg(short = 'p', long = "path")]
    path: Option<String>,
}

#[derive(Debug, Args)]
struct Issue {
    #[command(subcommand)]
    subcommands: IssueCommands,
}

#[derive(Debug, Subcommand)]
enum IssueCommands {
    Create(Create),
    List(List),
    Remove(Remove),
}

#[derive(Debug, Args)]
struct Create {
    // #[arg(short='t', long="title")]
    issue_title: String,
}

#[derive(Debug, Args)]
struct List {
    title: Option<String>,

    /// show all issues: contain closed.
    #[arg(long = "all")]
    all: bool,
}

#[derive(Debug, Args)]
struct Remove {
    id: u64,
}

#[derive(Debug, Args)]
struct Comment {
    #[arg(
        short = 'm',
        long = "message",
        conflicts_with_all = ["remove","hide"],
        // requires = "id"
    )]
    message: String,

    #[arg(long = "rm")]
    remove: bool,

    #[arg(long = "hide")]
    hide: bool,

    /// Specify the target issue ID. If not provided, and an issue is currently open, it will be used as the target.
    issue_id: Option<u64>,
}

#[derive(Debug, Args)]
struct Open {
    /// issue id
    issue_id: u64,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    Ok(())
}
