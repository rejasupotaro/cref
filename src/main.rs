#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

mod db_accessor;
mod commit;

use std::io::{self, Write};

const VERSION: &'static str = "0.0.1";

docopt!(Args derive Debug, "
Usage:
  cmsg <word>
  cmsg (-help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
");

fn main() {
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

    if !args.arg_word.is_empty() {
        db_accessor::access();
    }

    if args.flag_version {
        println!("{}", VERSION);
    }

    Ok(())
}
