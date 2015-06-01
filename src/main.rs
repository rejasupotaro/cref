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

use std::io::{self, Write};
use sqlite3::SqliteResult;
use model::repository::Repository;

const VERSION: &'static str = "0.0.1";

docopt!(Args derive Debug, "
Usage:
  cref
  cref import <repo>
  cref list
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
        Err(e) => abort(format!("oops!: {:?}", e).as_ref())
    }
}

pub fn abort(why: &str) {
    write!(&mut io::stderr(), "{}", why).unwrap();
    ::std::process::exit(1)
}

fn run(args: Args) -> SqliteResult<()> {
    trace!("{:?}", args);

    if args.cmd_import {
        let repo = args.arg_repo;
        let mut db = try!(db::Db::new("test.db"));
        let mut github = github::GitHub::new();
        let commits = github.fetch_commits(&repo);
        try!(db.insert_commits(&repo, commits));
    } else if args.cmd_list {
        let db = try!(db::Db::new("test.db"));
        let repositories = try!(db.select_repositories());
        repositories.iter().inspect(|repository| {
                println!("{:?}", repository);
            }).collect::<Vec<&Repository>>();
    } else if args.flag_version {
        println!("{}", VERSION);
    } else {
        let db = try!(db::Db::new("test.db"));
        let commits = try!(db.select_commits());
        let mut screen = view::Screen::new(commits);
        screen.draw();
    }

    Ok(())
}
