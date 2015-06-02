extern crate sqlite3;

use std::default::Default;

use self::sqlite3::Access;
use self::sqlite3::DatabaseConnection;
use self::sqlite3::Query;
use self::sqlite3::ResultRowAccess;
use self::sqlite3::SqliteResult;
use self::sqlite3::StatementUpdate;
use self::sqlite3::access;
use self::sqlite3::access::ByFilename;
use self::sqlite3::access::flags::OpenFlags;
use model::commit::Commit;
use model::repository::Repository;
use std::path::PathBuf;

pub struct Db {
    conn: DatabaseConnection,
}

impl Db {
    pub fn new(db_file: PathBuf) -> SqliteResult<Db> {
        let mut conn = try!(Db::open(Default::default(), &db_file.to_str().unwrap()));
        try!(conn.exec("PRAGMA foreign_keys = ON"));
        Ok(Db { conn: conn })
    }

    fn open(flags: OpenFlags, dbfile: &str) -> SqliteResult<DatabaseConnection> {
        let access = access::ByFilename { flags: flags, filename: dbfile };
        DatabaseConnection::new(access)
    }

    fn create_tables(&mut self) -> SqliteResult<()> {
        try!(self.conn.exec(create_repositories_table_query()));
        try!(self.conn.exec(create_commits_table_query()));
        Ok(())
    }

    pub fn insert_commits(&mut self, repository_name: &String, commits: Vec<Commit>) -> SqliteResult<()> {
        try!(self.create_tables());

        try!(self.conn.exec(insert_repository_query(&repository_name).as_ref()));
        let mut statement = try!(self.conn.prepare(select_repositories_by_name_query(repository_name).as_ref()));
        let mut repositories = vec!();
        try!(statement.query(
            &[], &mut |row| {
                repositories.push(Repository {
                    id: row.get(0),
                    name: row.get(1)
                });
                Ok(())
            }));
        let repository = repositories.get(0).unwrap();
        println!("insert repository {}", repository.name);

        for query in insert_commit_queries(repository.id, &commits) {
            let mut tx = try!(self.conn.prepare(query.as_ref()));
            let changes = try!(tx.update(&[]));
            assert_eq!(changes, 1);
        }
        println!("insert {} commits", &commits.len());

        Ok(())
    }

    pub fn select_repositories(&self) -> SqliteResult<Vec<Repository>> {
        let mut statement = try!(self.conn.prepare(select_repositories_query().as_ref()));
        let mut repositories = vec!();
        try!(statement.query(
            &[], &mut |row| {
                repositories.push(Repository {
                    id: row.get(0),
                    name: row.get(1)
                });
                Ok(())
            }));
        Ok(repositories)
    }

    pub fn select_commits(&self) -> SqliteResult<Vec<Commit>> {
        let mut statement = try!(self.conn.prepare(select_commits_query()));
        let mut commits = vec!();
        try!(statement.query(
            &[], &mut |row| {
                commits.push(Commit {
                    url: row.get(1),
                    message: row.get(2)
                });
                Ok(())
            }));
        Ok(commits)
    }

    pub fn delete_repository(&mut self, repository_name: String) -> SqliteResult<()> {
        try!(self.conn.exec(delete_repository_by_name_query(repository_name).as_ref()));
        Ok(())
    }
}

fn create_repositories_table_query() -> &'static str {
    "CREATE TABLE IF NOT EXISTS repositories (
        id      INTEGER PRIMARY KEY AUTOINCREMENT,
        name    VARCHAR NOT NULL UNIQUE
        )"
}

fn create_commits_table_query() -> &'static str {
    "CREATE TABLE IF NOT EXISTS commits (
        id            INTEGER PRIMARY KEY AUTOINCREMENT,
        url           VARCHAR NOT NULL UNIQUE,
        message       VARCHAR,
        repository_id INTERGER NOT NULL,
        FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE
        )"
}

fn insert_repository_query(repository_name: &String) -> String {
    format!("INSERT OR REPLACE INTO repositories (name) VALUES ('{}')", &repository_name)
}

fn insert_commit_queries(repository_id: i32, commits: &Vec<Commit>) -> Vec<String> {
    commits.iter().map(|commit| {
            format!("INSERT OR REPLACE INTO commits (url, message, repository_id) VALUES ('{}', '{}', {})",
                commit.url,
                commit.message.replace("\n", " ").replace("'", ""), // TODO: impl exact escape later
                repository_id)
        }).collect::<Vec<String>>()
}

fn select_repositories_by_name_query(repository_name: &String) -> String {
    format!("SELECT * FROM repositories WHERE name='{}'", repository_name)
}

fn select_repositories_query() -> String {
    format!("SELECT * FROM repositories")
}

fn select_commits_query() -> &'static str {
    "SELECT * FROM commits"
}

fn delete_repository_by_name_query(repository_name: String) -> String {
    format!("DELETE FROM repositories WHERE name='{}'", repository_name)
}
