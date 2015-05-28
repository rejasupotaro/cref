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

use model::commit::Commit;

pub fn access() {
    let ok = |flags, dbfile| Some(access::ByFilename { flags: flags, filename: dbfile });

    fn use_access<A: Access>(access: A) -> SqliteResult<Vec<Commit>> {
        let mut conn = try!(DatabaseConnection::new(access));
        insert_commit(&mut conn)
    }

    fn lose(why: &str) {
        write!(&mut io::stderr(), "{}", why).unwrap();
        ::std::process::exit(1)
    }

    let dbfile = "test.db";
    match ok(Default::default(), dbfile) {
        Some(a) => match use_access(a) {
            Ok(x) => println!("Ok: {:?}", x),
            Err(oops) => lose(format!("oops!: {:?}", oops).as_ref())
        },
        None => lose("usage")
    }
}

fn insert_commit(conn: &mut DatabaseConnection) -> SqliteResult<Vec<Commit>> {
    try!(conn.exec("DROP TABLE IF EXISTS commits"));
    try!(conn.exec("CREATE TABLE commits (
                        id    SERIAL PRIMARY KEY,
                        name  VARCHAR NOT NULL
                    )"));

    {
        let mut tx = try!(conn.prepare("INSERT INTO commits (id, name)
                           VALUES (0, 'Dan')"));
        let changes = try!(tx.update(&[]));
        assert_eq!(changes, 1);
    }

    let mut stmt = try!(conn.prepare("SELECT id, name FROM commits"));

    let mut ppl = vec!();
    try!(stmt.query(
        &[], &mut |row| {
            ppl.push(Commit {
                id: row.get(0),
                name: row.get(1)
            });
            Ok(())
        }));
    Ok(ppl)
}
