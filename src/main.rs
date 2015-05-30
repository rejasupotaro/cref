#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;

mod db;
mod http;
mod model;

use std::io::{self, Write};

const VERSION: &'static str = "0.0.1";

docopt!(Args derive Debug, "
Usage:
  cmsg import <repo>
  cmsg <word>
  cmsg (-help | --version)

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

fn run(args: Args) -> Result<(), String> {
    println!("{:?}", args);

    if args.cmd_import {
        http::access(args.arg_repo);
    }

    if !args.arg_word.is_empty() {
        db::access();
    }

    if args.flag_version {
        println!("{}", VERSION);
    }

    Ok(())
}
