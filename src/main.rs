use clap::{Parser, Subcommand};
use local_issues_lib::{self, Project};
use std::{env, fmt::Display, io, path::PathBuf};

#[derive(Debug)]
enum Error {
    LocalIssueError(local_issues_lib::Error),
    IoError(io::Error),
    NotInitialized,
    NeedId,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LocalIssueError(e) => write!(f, "lib error: {}", e),
            Error::IoError(e) => write!(f, "{}", e),
            Error::NotInitialized => write!(f, "not initialized"),
            Error::NeedId => write!(f, "Need ID"),
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// initialized potato issues tracker in /folder_path
    #[clap(arg_required_else_help = true)]
    Init(Init),

    List(List),

    /// control issues
    Issue(Issue),

    /// commit message or other ctrl
    Commit(Commit),
}

#[derive(Debug, Parser)]
struct Init {
    // /// project title, when is empty, set folder name.
    // title: Option<String>,
    /// target dir path. when it's empty, set current dir.
    dir: Option<String>,
}

/// show list: issues or commit messages
#[derive(Debug, Parser)]
struct List {
    /// show commit messages list.
    #[arg(short = 'c', long = "cmtmsg")]
    is_cmtmsg: bool,

    /// when show commit messages, need to choose which issue by id.
    /// require `--cmtmsg`
    #[arg(short = 'i', long = "id", requires = "is_cmtmsg")]
    issue_id: Option<u64>,

    /// show all(include closed contents)
    #[arg(long = "all")]
    all: bool,

    /// show list as oneline
    #[arg(long = "oneline")]
    oneline: bool,
}

#[derive(Debug, Parser)]
struct Issue {
    /// target id for tags, etc...
    target_id: Option<u64>,

    /// new issue
    #[arg(short = 'n', long = "new")]
    new_issue_name: Option<String>,

    /// add tags to issue_name. requires TARGET_ID. split tags by `,`
    #[arg(
        short = 't',
        long = "tags",
        requires = "target_id",
        value_delimiter = ','
    )]
    tags: Option<Vec<String>>,

    /// delete issue by id, requires TARGET_ID
    #[arg(short = 'd', long = "delete", requires = "target_id")]
    delete: bool,
}

#[derive(Debug, Parser)]
struct Commit {
    /// commit message by issue_id
    #[arg(short = 'm', long = "message", requires = "issue_id")]
    message: Option<String>,

    /// delete by id
    #[arg(short = 'd', long = "delete", requires = "issue_id")]
    delete: bool,

    /// target id for tags, etc...
    issue_id: Option<u64>,
}

fn work_path() -> PathBuf {
    env::current_dir().unwrap()
}

fn folder_name() -> String {
    env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

fn main() -> Result<(), Error> {
    let cli = Args::parse();

    let db = Project::open(folder_name(), work_path()).ok();

    match cli.subcommand {
        Commands::Init(init) => {
            if db.is_some() {
                println!("this folder is initialized.");
                return Ok(());
            }
            let title = folder_name();
            let path = match init.dir {
                Some(v) => PathBuf::from(v),
                None => env::current_dir().unwrap(),
            };
            let _a = Project::open(title, &path).map_err(Error::LocalIssueError)?;

            println!(
                "initialized potato issues tracker in {}",
                path.to_str().unwrap()
            );
        }
        Commands::List(list) => match db {
            Some(db) => {
                if list.is_cmtmsg {
                    let issue_id = match list.issue_id {
                        Some(id) => id,
                        None => return Err(Error::NeedId),
                    };
                    let _list = db.get_opened_issue_id();
                } else if list.oneline {
                    println!("{}", db.oneline_fmt());
                } else if list.all {
                    println!("{}", db);
                } else {
                    println!("{}", db.filterd_string(db.get_opened_issue_id().unwrap()));
                }
            }
            None => {
                return Err(Error::NotInitialized);
            }
        },
        Commands::Issue(issue) => todo!(),
        Commands::Commit(commit) => todo!(),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::folder_name;

    #[test]
    fn name_test() {
        println!("{}", folder_name());
    }
}
