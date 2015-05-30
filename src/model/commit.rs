use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Commit {
    pub url: String,
    pub message: String,
}

impl ToJson for Commit {
    fn to_json(&self) -> Json {
        let str = json::encode(&self).unwrap();
        Json::from_str(str.as_ref()).unwrap()
    }
}
