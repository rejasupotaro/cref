use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct CommitEntity {
    url: String,
}

impl CommitEntity {
    pub fn from_str(json_str: String) -> Vec<CommitEntity> {
        json::decode(&json_str).unwrap()
    }
}

impl ToJson for CommitEntity {
    fn to_json(&self) -> Json {
        let str = json::encode(&self).unwrap();
        Json::from_str(str.as_ref()).unwrap()
    }
}
