#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Commit {
    pub url: String,
    pub message: String,
}
