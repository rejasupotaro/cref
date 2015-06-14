use model::commit::Commit;

pub fn enable_foreign_key() -> &'static str {
    "PRAGMA foreign_keys = ON"
}

pub fn create_repositories_table() -> &'static str {
    "CREATE TABLE IF NOT EXISTS repositories (
        id      INTEGER PRIMARY KEY AUTOINCREMENT,
        name    VARCHAR NOT NULL UNIQUE
        )"
}

pub fn create_commits_table() -> &'static str {
    "CREATE TABLE IF NOT EXISTS commits (
        id            INTEGER PRIMARY KEY AUTOINCREMENT,
        url           VARCHAR NOT NULL UNIQUE,
        message       VARCHAR,
        repository_id INTERGER NOT NULL,
        FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE
        )"
}

pub fn insert_repository(repository_name: &String) -> String {
    format!("INSERT OR REPLACE INTO repositories (name) VALUES ('{}')", &repository_name)
}

pub fn insert_commit(repository_id: i32, commits: &Vec<Commit>) -> Vec<String> {
    commits.iter().map(|commit| {
        format!("INSERT OR REPLACE INTO commits (url, message, repository_id) VALUES ('{}', '{}', {})",
            commit.url,
            commit.message.replace("\n", " ").replace("'", ""), // TODO: impl exact escape later
            repository_id)
    }).collect::<Vec<String>>()
}

pub fn select_repositories_by_name(repository_name: &String) -> String {
    format!("SELECT * FROM repositories WHERE name='{}'", repository_name)
}

pub fn select_repositories() -> String {
    format!("SELECT * FROM repositories")
}

pub fn select_commits() -> &'static str {
    "SELECT * FROM commits"
}

pub fn delete_repository_by_name(repository_name: String) -> String {
    format!("DELETE FROM repositories WHERE name='{}'", repository_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_repository() {
        assert_eq!("INSERT OR REPLACE INTO repositories (name) VALUES ('rejasupotaro/cref')",
            insert_repository(&"rejasupotaro/cref".to_string()));
    }

    #[test]
    fn test_select_repositories_by_name() {
        assert_eq!("SELECT * FROM repositories WHERE name='rejasupotaro/cref'",
            select_repositories_by_name(&"rejasupotaro/cref".to_string()));
    }

    #[test]
    fn test_delete_repository_by_name() {
        assert_eq!("DELETE FROM repositories WHERE name='rejasupotaro/cref'",
            delete_repository_by_name("rejasupotaro/cref".to_string()));
    }
}
