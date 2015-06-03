extern crate log;
extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate sqlite3;

use std::env;
use std::fs;
use std::path::PathBuf;
use super::db;
use super::github;
use super::view;
use super::Args;
use super::abort;

const VERSION: &'static str = "0.0.1";

pub struct Cref {
    db: db::Db
}

impl Cref {
    pub fn new() -> Cref {
        create_cref_dir();
        match db::Db::new(db_file()) {
            Ok(db) => Cref { db: db },
            Err(e) => panic!(e.to_string())
        }
    }

    pub fn run(&mut self, args: Args) {
        trace!("{:?}", args);

        if args.cmd_import { // cref import <repo>
            self.execute_import(args.arg_import_repo);
        } else if args.cmd_list { // cref list
            self.execute_list();
        } else if args.cmd_update { // cref update
            self.execute_update(args.arg_update_repo);
        } else if args.cmd_delete { // cref delete <repo>
            self.execute_delete(args.arg_delete_repo);
        } else if args.flag_version { // cref -v
            println!("{}", VERSION);
        } else { // cref
            self.execute();
        }
    }

    fn execute_import(&mut self, repository_names: Vec<String>) {
        let mut github = github::GitHub::new();
        repository_names.iter().map(|repository_name| {
                let commits = github.fetch_commits(&repository_name);
                self.db.insert_commits(&repository_name, commits);
            }).collect::<Vec<_>>();
    }

    fn execute_list(&self) {
        match self.db.select_repositories() {
            Ok(repositories) => {
                repositories.iter().map(|repository| {
                        println!("{:?}", repository);
                    }).collect::<Vec<_>>();
            },
            Err(e) => abort(e.to_string())
        }
    }

    fn execute_update(&self, repository_names: Vec<String>) {
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
                match self.db.select_repositories() {
                    Ok(repositories) => {
                        let all_repository_names = repositories.iter().map(|repository| {
                                repository.name.clone()
                            }).collect::<Vec<String>>();
                        update(all_repository_names);
                    },
                    Err(e) => abort(e.to_string())
                }
            },
            _ => {
                update(repository_names);
            }
        }
    }

    fn execute_delete(&mut self, repository_name: String) {
        match self.db.delete_repository(repository_name) {
            Ok(()) => {},
            Err(e) => abort(e.to_string())
        }
    }

    fn execute(&self) {
        match self.db.select_commits() {
            Ok(commits) => {
                let mut screen = view::Screen::new(commits);
                screen.draw();
            },
            Err(e) => abort(e.to_string())
        }
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
