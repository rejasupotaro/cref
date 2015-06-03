extern crate log;
extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate sqlite3;

use std::env;
use std::fs;
use std::path::PathBuf;
use sqlite3::SqliteResult;
use super::db;
use super::github;
use super::view;
use super::Args;
use super::abort;
use super::errors::CrefError;

const VERSION: &'static str = "0.0.1";

pub struct Cref {
    db: db::Db
}

impl Cref {
    pub fn new() -> Cref {
        create_cref_dir();
        match db::Db::new(db_file()) {
            Ok(db) => Cref { db: db},
            Err(e) => panic!(e.to_string())
        }
    }

    pub fn run(&self, args: Args) -> SqliteResult<()> {
        trace!("{:?}", args);

        if args.cmd_import { // cref import <repo>
            try!(self.execute_import(args.arg_import_repo));
        } else if args.cmd_list { // cref list
            try!(self.execute_list());
        } else if args.cmd_update { // cref update
            try!(self.execute_update(args.arg_update_repo));
        } else if args.cmd_delete { // cref delete <repo>
            try!(self.execute_delete(args.arg_delete_repo));
        } else if args.flag_version { // cref -v
            println!("{}", VERSION);
        } else { // cref
            try!(self.execute());
        }

        Ok(())
    }

    fn execute_import(&self, repository_names: Vec<String>) -> SqliteResult<()> {
        let mut db = try!(db::Db::new(db_file()));
        let mut github = github::GitHub::new();
        repository_names.iter().map(|repository_name| {
                let commits = github.fetch_commits(&repository_name);
                db.insert_commits(&repository_name, commits);
            }).collect::<Vec<_>>();
        Ok(())
    }

    fn execute_list(&self) -> SqliteResult<()> {
        let repositories = try!(self.db.select_repositories());
        repositories.iter().map(|repository| {
                println!("{:?}", repository);
            }).collect::<Vec<_>>();
        Ok(())
    }

    fn execute_update(&self, repository_names: Vec<String>) -> SqliteResult<()> {
        let update = |repository_names: Vec<String>| {
            match db::Db::new(db_file()) {
                Ok(mut db) => {
                    let mut github = github::GitHub::new();

                    repository_names.iter().map(|repository_name| {
                            let commits = github.fetch_commits(&repository_name);
                            db.insert_commits(&repository_name, commits);
                        }).collect::<Vec<_>>();
                },
                Err(e) => abort(e.to_string())
            }
        };

        match repository_names.len() {
            0 => {
                let db = try!(db::Db::new(db_file()));
                let all_repository_names = try!(db.select_repositories()).iter().map(|repository| {
                        repository.name.clone()
                    }).collect::<Vec<String>>();
                update(all_repository_names);
            },
            _ => {
                update(repository_names);
            }
        }
        Ok(())
    }

    fn execute_delete(&self, repository_name: String) -> SqliteResult<()> {
        let mut db = try!(db::Db::new(db_file()));
        try!(db.delete_repository(repository_name));
        Ok(())
    }

    fn execute(&self) -> SqliteResult<()> {
        let db = try!(db::Db::new(db_file()));
        let commits = try!(db.select_commits());
        let mut screen = view::Screen::new(commits);
        screen.draw();
        Ok(())
    }
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
