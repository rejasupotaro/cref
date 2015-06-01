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

pub struct Db {
    conn: DatabaseConnection,
}

impl Db {
    pub fn new(dbfile: &str) -> SqliteResult<Db> {
        let conn = try!(Db::open(Default::default(), dbfile));
        Ok(Db { conn: conn })
    }

    fn open(flags: OpenFlags, dbfile: &str) -> SqliteResult<DatabaseConnection> {
        let access = access::ByFilename { flags: flags, filename: dbfile };
        DatabaseConnection::new(access)
    }

    fn create_table(&mut self) -> SqliteResult<()> {
        try!(self.conn.exec(create_table_query()));
        Ok(())
    }

    pub fn insert_commits(&mut self, commits: Vec<Commit>) -> SqliteResult<()> {
        self.create_table();

        let queries = insert_queries(commits);
        for query in queries {
            let mut tx = try!(self.conn.prepare(query.as_ref()));
            let changes = try!(tx.update(&[]));
            assert_eq!(changes, 1);
        }
        Ok(())
    }

    fn filter_commits(&self, commits: Vec<Commit>, word: String) -> Vec<Commit> {
        let mut result = vec!();
        for commit in commits {
            if commit.message.contains(&word) {
                result.push(commit);
            }
        }
        result
    }

    pub fn fetch_commits(&self, word: String) -> SqliteResult<Vec<Commit>> {
        let mut statement = try!(self.conn.prepare(select_query()));
        let mut commits = vec!();
        try!(statement.query(
            &[], &mut |row| {
                commits.push(Commit {
                    url: row.get(1),
                    message: row.get(2),
                });
                Ok(())
            }));

        Ok(self.filter_commits(commits, word))
    }
}

fn create_table_query() -> &'static str {
    "CREATE TABLE IF NOT EXISTS commits (
        id      INTEGER PRIMARY KEY AUTOINCREMENT,
        url     VARCHAR NOT NULL UNIQUE,
        message VARCHAR
        )"
}

fn insert_queries(commits: Vec<Commit>) -> Vec<String> {
    commits.iter().map(|commit| {
            format!("INSERT OR REPLACE INTO commits (url, message) VALUES ('{}', '{}')",
                commit.url,
                commit.message.replace("\n", " ").replace("'", "")) // TODO: impl exact escape later
        }).collect::<Vec<String>>()
}

fn select_query() -> &'static str {
    "SELECT id, url, message FROM commits"
}
