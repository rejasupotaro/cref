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
mod cref;
mod errors;

use docopt::Docopt;
use std::io::{self, Write};
use cref::Cref;

static USAGE: &'static str = "
Usage:
  cref
  cref import <import-repo>...
  cref list
  cref update [<update-repo>...]
  cref delete <delete-repo>
  cref (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
";

#[derive(RustcDecodable, Debug)]
pub struct Args {
    cmd_import: bool,
    flag_version: bool,
    arg_import_repo: Vec<String>,
    cmd_update: bool,
    cmd_delete: bool,
    arg_delete_repo: String,
    flag_help: bool,
    cmd_list: bool,
    arg_update_repo: Vec<String>
}

extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut cref = Cref::new();
    cref.run(args);
}

pub fn abort(why: String) {
    write!(&mut io::stderr(), "{}", why).unwrap();
    ::std::process::exit(1)
}
