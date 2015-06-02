extern crate sqlite3;

mod query;

use std::default::Default;
use std::path::PathBuf;
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

pub struct Db {
    conn: DatabaseConnection,
}

impl Db {
    pub fn new(db_file: PathBuf) -> SqliteResult<Db> {
        let mut conn = try!(Db::open(Default::default(), &db_file.to_str().unwrap()));
        try!(conn.exec(query::enable_foreign_key()));
        Ok(Db { conn: conn })
    }

    fn open(flags: OpenFlags, dbfile: &str) -> SqliteResult<DatabaseConnection> {
        let access = access::ByFilename { flags: flags, filename: dbfile };
        DatabaseConnection::new(access)
    }

    fn create_tables(&mut self) -> SqliteResult<()> {
        try!(self.conn.exec(query::create_repositories_table()));
        try!(self.conn.exec(query::create_commits_table()));
        Ok(())
    }

    pub fn insert_commits(&mut self, repository_name: &String, commits: Vec<Commit>) -> SqliteResult<()> {
        try!(self.create_tables());

        try!(self.conn.exec(query::insert_repository(&repository_name).as_ref()));
        let mut statement = try!(self.conn.prepare(query::select_repositories_by_name(repository_name).as_ref()));
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

        for query in query::insert_commit(repository.id, &commits) {
            let mut tx = try!(self.conn.prepare(query.as_ref()));
            let changes = try!(tx.update(&[]));
            assert_eq!(changes, 1);
        }
        println!("insert {} commits", &commits.len());

        Ok(())
    }

    pub fn select_repositories(&self) -> SqliteResult<Vec<Repository>> {
        let mut statement = try!(self.conn.prepare(query::select_repositories().as_ref()));
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
        let mut statement = try!(self.conn.prepare(query::select_commits()));
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
        try!(self.conn.exec(query::delete_repository_by_name(repository_name).as_ref()));
        Ok(())
    }
}
