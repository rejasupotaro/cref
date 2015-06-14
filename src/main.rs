#![feature(plugin)]
#![plugin(docopt_macros)]
#[macro_use]

extern crate log;
extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate sqlite3;
extern crate env_logger;

mod db;
mod github;
mod model;
mod cref;
mod errors;
mod screen;

use docopt::Docopt;
use std::io::{self, Write};
use cref::Cref;

docopt!(Args derive Debug, "
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
");

fn main() {
    env_logger::init().unwrap();

    let args = Args::docopt()
            .decode()
            .unwrap_or_else(|e| e.exit());

    let mut cref = Cref::new();
    cref.run(args);
}

pub fn abort(why: String) {
    write!(&mut io::stderr(), "{}", why).unwrap();
    ::std::process::exit(1)
}
