use chrono::Local;
use clap::{Args, Parser, Subcommand, command};
use local_issues_lib::{self, DbProject, Project};
use std::{
    env,
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

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
            // Error::NotFound(e) => write!(f, "not found: {}", e),
            // Error::PathNotFound(_) => write!(f, "path not found"),
        }
    }
}

impl Error {
    fn is_file_zero(&self) -> bool {
        match self {
            Error::LocalIssue(local_issues_lib::Error::DbError(error)) => error.is_file_is_zero(),
            _ => false,
        }
    }
}

#[derive(Debug, Parser)]
#[command(propagate_version = true)]
#[command(name = "cpls", version = VERSION)]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// initialize dir (or -p path folder)
    Init(Init),
    /// create new issue
    Create(Create),
    /// close issue
    Delete(Delete),
    /// show list;  issues and comments in issues
    List(List),
    /// add comment to issue by issue num; get by `list`command
    Comment(Comment),
}

#[derive(Debug, Args)]
struct Init {
    name: Option<String>,

    #[arg(short = 'p', long = "path")]
    path: Option<String>,
}

#[derive(Debug, Args)]
struct Create {
    name: String,
}

#[derive(Debug, Args)]
struct Delete {
    issue_num: u64,

    #[arg(short = 'u', long = "unresolved")]
    unresolved: bool,
}

#[derive(Debug, Args)]
struct List {}

#[derive(Debug, Args)]
struct Comment {
    issue_num: u64,
    comment: String,
}

/// return dir name
fn project_name<T: AsRef<str>>(dir: &Path, def: T) -> String {
    dir.file_stem()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or(def.as_ref().to_string())
        .trim()
        .to_string()
}

/// if failed to get current dir name, use datetime as project name
fn init_f(work_path: PathBuf, init: Init) -> Result<(), Error> {
    let title = init
        .name
        .unwrap_or(project_name(&work_path, Local::now().to_rfc2822()));

    // if init path (-p) exists, bound it, otherwise bound work_path
    let work_path = match init.path {
        Some(arg_path) => {
            let p = PathBuf::from(arg_path);
            if p.exists() { p } else { work_path }
        }
        None => work_path,
    };

    if cfg!(test) {
        println!("{:?}", work_path);
    }

    // if loaded file is empty bound empty Project struct.
    Project::open(&work_path)
        .or_else(|e| {
            if e.is_file_is_zero() {
                Ok(Project::new(title, &work_path))
            } else {
                Err(e)
            }
        })
        .map(|_| ())
        .map_err(Error::LocalIssue)
}

fn main() -> Result<(), Error> {
    let current_dir = env::current_dir().map_err(Error::Io)?;
    let work_path = if cfg!(test) || cfg!(debug_assertions) {
        current_dir.join("tests").join("test")
    } else {
        current_dir
    };

    if cfg!(test) || cfg!(debug_assertions) {
        println!("{:?}", work_path);
    }

    let cli = Cli::parse();

    println!("{:?}", cli);
    let result = match cli.subcommand {
        Commands::Init(init) => init_f(work_path, init),
        Commands::Create(create) => {
            let mut db = Project::open(&work_path).map_err(Error::LocalIssue)?;
            let issue_name = create.name;
            db.add_issue(issue_name);
            db.save().map_err(Error::LocalIssue)
        }
        Commands::Delete(delete) => {
            let mut db = Project::open(&work_path).map_err(Error::LocalIssue)?;

            //                                                maybe unresolved flag isn't much use
            db.to_close_issue(delete.issue_num, !delete.unresolved);
            db.save().map_err(Error::LocalIssue)
        }
        Commands::List(_list) => {
            let db = Project::open(&work_path).map_err(Error::LocalIssue)?;
            println!("{}", db);
            Ok(())
        }
        Commands::Comment(comment) => {
            let mut db = Project::open(&work_path).map_err(Error::LocalIssue)?;
            // let id = comment.issue_num;
            db.commit(comment.issue_num, comment.comment);
            db.save().map_err(Error::LocalIssue)
        }
    };

    match result {
        Ok(_) => {
            println!("success");
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Error, Init, init_f};
    use std::env;

    #[test]
    fn test_init() -> Result<(), Error> {
        let mut work_path = env::current_dir().map_err(Error::Io)?;
        work_path.push("tests");
        work_path.push("test");

        let ini = Init {
            name: Some("test".to_string()),
            path: None,
        };

        let a = init_f(work_path, ini);
        println!("{:?}", a);
        Ok(())
    }
}
