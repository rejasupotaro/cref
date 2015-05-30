use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use model::commit::Commit;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct CommitEntity {
    pub commit: Commit,
}

impl ToJson for CommitEntity {
    fn to_json(&self) -> Json {
        let str = json::encode(&self).unwrap();
        Json::from_str(str.as_ref()).unwrap()
    }
}
