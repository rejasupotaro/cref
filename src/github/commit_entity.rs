use model::commit::Commit;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct CommitEntity {
    pub commit: Commit,
}
