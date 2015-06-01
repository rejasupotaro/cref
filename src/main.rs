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

use std::io::{self, Write};
use sqlite3::SqliteResult;

const VERSION: &'static str = "0.0.1";

docopt!(Args derive Debug, "
Usage:
  cref import <repo>
  cref <word>
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
        Err(err) => {
            write!(&mut io::stderr(), "{}", err).unwrap();
            ::std::process::exit(1)
        }
    }
}

pub fn abort(why: &str) {
    write!(&mut io::stderr(), "{}", why).unwrap();
    ::std::process::exit(1)
}

fn run(args: Args) -> SqliteResult<()> {
    trace!("{:?}", args);

    if args.cmd_import {
        let mut db = try!(db::Db::new("test.db"));
        let mut github = github::GitHub::new();
        let commits = github.fetch_commits(args.arg_repo);
        db.insert_commits(commits);
    }

    if !args.arg_word.is_empty() {
        let db = try!(db::Db::new("test.db"));
        match db.fetch_commits(args.arg_word) {
            Ok(commits) => println!("{:?}", commits),
            Err(e) => abort(format!("oops!: {:?}", e).as_ref())
        }
    }

    if args.flag_version {
        println!("{}", VERSION);
    }

    Ok(())
}
