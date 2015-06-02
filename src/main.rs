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

    let args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

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

    if args.cmd_import { // cref import <repo>
        try!(execute_import(args.arg_repo));
    } else if args.cmd_list { // cref list
        try!(execute_list());
    } else if args.cmd_update { // cref update
        try!(execute_update());
    } else if args.cmd_delete { // cref delete <repo>
        try!(execute_delete(args.arg_repo));
    } else if args.flag_version { // cref -v
        println!("{}", VERSION);
    } else { // cref
        try!(execute());
    }

    Ok(())
}

fn execute_import(repository_name: String) -> SqliteResult<()> {
    let mut db = try!(db::Db::new(db_file()));
    let mut github = github::GitHub::new();
    let commits = github.fetch_commits(&repository_name);
    db.insert_commits(&repository_name, commits)
}

fn execute_list() -> SqliteResult<()> {
    let db = try!(db::Db::new(db_file()));
    let repositories = try!(db.select_repositories());
    repositories.iter().inspect(|repository| {
            println!("{:?}", repository);
        }).collect::<Vec<_>>();
    Ok(())
}

fn execute_update() -> SqliteResult<()> {
    let mut db = try!(db::Db::new(db_file()));
    let mut github = github::GitHub::new();
    try!(db.select_repositories()).iter().map(|repository| {
            let commits = github.fetch_commits(&repository.name);
            db.insert_commits(&repository.name, commits);
        }).collect::<Vec<_>>();
    Ok(())
}

fn execute_delete(repository_name: String) -> SqliteResult<()> {
    let mut db = try!(db::Db::new(db_file()));
    try!(db.delete_repository(repository_name));
    Ok(())
}

fn execute() -> SqliteResult<()> {
    let db = try!(db::Db::new(db_file()));
    let commits = try!(db.select_commits());
    let mut screen = view::Screen::new(commits);
    screen.draw();
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
