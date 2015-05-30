extern crate sqlite3;

use std::default::Default;
use std::io::{self, Write};

use self::sqlite3::{
    Access,
    DatabaseConnection,
    Query,
    ResultRowAccess,
    SqliteResult,
    StatementUpdate,
};
use self::sqlite3::access;
use self::sqlite3::access::ByFilename;
use self::sqlite3::access::flags::OpenFlags;

use model::commit::Commit;

fn open(flags: OpenFlags, dbfile: &str) -> Option<ByFilename> {
    Some(access::ByFilename { flags: flags, filename: dbfile })
}

fn lose(why: &str) {
    write!(&mut io::stderr(), "{}", why).unwrap();
    ::std::process::exit(1)
}

fn insert_query(commits: Vec<Commit>) -> Vec<String> {
    let mut queries = vec!();
    for commit in commits {
        let query = format!("INSERT OR REPLACE INTO commits (url, message) VALUES ('{}', '{}')",
            commit.url,
            commit.message.replace("\n", " ").replace("'", "")); // TODO: impl exact escape later
            trace!("{}", query);
        queries.push(query);
    }
    queries
}

fn create_table(conn: &mut DatabaseConnection) -> SqliteResult<()> {
    try!(conn.exec("CREATE TABLE IF NOT EXISTS commits (
                        id    INTEGER PRIMARY KEY AUTOINCREMENT,
                        url  VARCHAR NOT NULL UNIQUE,
                        message VARCHAR
                    )"));
    Ok(())
}

fn try_insert<A: Access>(access: A, commits: Vec<Commit>) -> SqliteResult<()> {
    let mut conn = try!(DatabaseConnection::new(access));
    create_table(&mut conn);

    let queries = insert_query(commits);
    for query in queries {
        let mut tx = try!(conn.prepare(query.as_ref()));
        let changes = try!(tx.update(&[]));
        assert_eq!(changes, 1);
    }
    Ok(())
}

pub fn insert_commits(commits: Vec<Commit>) {
    let dbfile = "test.db";
    match open(Default::default(), dbfile) {
        Some(access) => match try_insert(access, commits) {
            Ok(x) => println!("Ok: {:?}", x),
            Err(oops) => lose(format!("oops!: {:?}", oops).as_ref())
        },
        None => lose("usage")
    }
}

fn try_select<A: Access>(access: A) -> SqliteResult<Vec<Commit>> {
    let mut conn = try!(DatabaseConnection::new(access));
    let mut stmt = try!(conn.prepare("SELECT id, url, message FROM commits"));

    let mut commits = vec!();
    try!(stmt.query(
        &[], &mut |row| {
            commits.push(Commit {
                url: row.get(1),
                message: row.get(2),
            });
            Ok(())
        }));
    Ok(commits)
}

pub fn select_commits(word: String) {
    let dbfile = "test.db";
    match open(Default::default(), dbfile) {
        Some(access) => match try_select(access) {
            Ok(commits) => {
                commits.iter()
                    .filter(|commit| commit.message.contains(&word))
                    .inspect(|commit| println!("{}", commit.message))
                    .collect::<Vec<&Commit>>();
            },
            Err(oops) => lose(format!("oops!: {:?}", oops).as_ref())
        },
        None => lose("usage")
    }
}
