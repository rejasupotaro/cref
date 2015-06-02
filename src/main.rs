#![feature(plugin)]
#![plugin(docopt_macros)]
#[macro_use]

extern crate log;
extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate sqlite3;

mod db;
mod github;
mod model;
mod view;

use std::env;
use std::io::{self, Write};
use std::fs;
use std::path::PathBuf;
use sqlite3::SqliteResult;
use model::repository::Repository;

const VERSION: &'static str = "0.0.1";

docopt!(Args derive Debug, "
Usage:
  cref
  cref import <repo>
  cref list
  cref update
  cref delete <repo>
  cref (-help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
");

extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    match run(args) {
        Ok(_) => {},
        Err(e) => abort(e.to_string())
    }
}

pub fn abort(why: String) {
    write!(&mut io::stderr(), "{}", why).unwrap();
    ::std::process::exit(1)
}

fn run(args: Args) -> SqliteResult<()> {
    trace!("{:?}", args);

    create_cref_dir();
    let mut db = try!(db::Db::new(db_file()));

    if args.cmd_import { // cref import <repo>
        let repository_name = args.arg_repo;
        let mut github = github::GitHub::new();
        let commits = github.fetch_commits(&repository_name);
        try!(db.insert_commits(&repository_name, commits));
    } else if args.cmd_list { // cref list
        let repositories = try!(db.select_repositories());
        repositories.iter().inspect(|repository| {
                println!("{:?}", repository);
            }).collect::<Vec<&Repository>>();
    } else if args.cmd_update { // cref update
        let mut github = github::GitHub::new();
        try!(db.select_repositories()).iter().map(|repository| {
                let commits = github.fetch_commits(&repository.name);
                db.insert_commits(&repository.name, commits);
            }).collect::<Vec<_>>();
    } else if args.cmd_delete { // cref delete <repo>
        let repository_name = args.arg_repo;
        try!(db.delete_repository(repository_name));
    } else if args.flag_version { // cref -v
        println!("{}", VERSION);
    } else { // cref
        let commits = try!(db.select_commits());
        let mut screen = view::Screen::new(commits);
        screen.draw();
    }

    Ok(())
}

fn db_file() -> PathBuf {
    cref_dir().join("cref.db")
}

fn cref_dir() -> PathBuf {
    env::home_dir().map(|p| p.join(".cref")).unwrap()
}

fn create_cref_dir() {
    fs::create_dir(cref_dir());
}
