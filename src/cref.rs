extern crate log;
extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate sqlite3;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::cell::RefCell;
use screen::Screen;
use super::db::Db;
use super::github::GitHub;
use super::Args;
use super::abort;
use super::model::commit::Commit;

const VERSION: &'static str = "0.0.1";

pub struct Cref {
    db: Arc<RefCell<Db>>
}

impl Cref {
    pub fn new() -> Cref {
        create_cref_dir();
        match Db::new(db_file()) {
            Ok(db) => Cref { db: Arc::new(RefCell::new(db)) },
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

    fn execute_import(&self, repository_names: Vec<String>) {
        let (tx, rx) = mpsc::channel();
        for i in 0..repository_names.len() {
            let name = repository_names.get(i).unwrap().to_string();
            let tx = tx.clone();
            thread::spawn(move || {
                let mut github = GitHub::new();
                let commits = github.fetch_commits(&name);
                tx.send((name.to_string(), commits));
            });
        }

        for _ in 0..repository_names.len() {
            let (name, commits) = rx.recv().unwrap();
            self.db.borrow_mut().insert_commits(&name, commits);
        }
    }

    fn execute_list(&self) {
        match self.db.borrow_mut().select_repositories() {
            Ok(repositories) => {
                for repository in repositories.iter() {
                    println!("{:?}", repository);
                }
            },
            Err(e) => abort(e.to_string())
        }
    }

    fn execute_update(&self, repository_names: Vec<String>) {
        match repository_names.len() {
            0 => {
                match self.db.borrow().select_repositories() {
                    Ok(repositories) => {
                        let all_repository_names = repositories.iter().map(|repository| {
                                repository.name.clone()
                        }).collect::<Vec<String>>();
                        self.execute_import(all_repository_names);
                    },
                    Err(e) => abort(e.to_string())
                }
            },
            _ => {
                self.execute_import(repository_names);
            }
        }
    }

    fn execute_delete(&mut self, repository_name: String) {
        match self.db.borrow_mut().delete_repository(repository_name) {
            Ok(()) => {},
            Err(e) => abort(e.to_string())
        }
    }

    fn execute(&self) {
        match self.db.borrow_mut().select_commits() {
            Ok(commits) => {
                let mut screen = Screen::new(commits);
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
