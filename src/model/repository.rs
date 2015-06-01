#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Repository {
    pub id: i32,
    pub name: String
}
