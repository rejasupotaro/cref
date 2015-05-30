extern crate hyper;

mod commit_entity;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::UserAgent;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use self::commit_entity::CommitEntity;
use model::commit::Commit;

pub fn fetch_commits(repo: String) -> Vec<Commit> {
    let mut client = Client::new();
    let mut res = client.get("https://api.github.com/repos/rejasupotaro/kvs-schema/commits")
        .header(UserAgent("cmsg".to_owned()))
        .send();

    match res {
        Ok(mut r) => {
            trace!("Response: {:?}", r);

            let mut body = String::new();
            r.read_to_string(&mut body).unwrap();
            let entities: Vec<CommitEntity> = json::decode(&body).unwrap();

            let mut models: Vec<Commit> = Vec::new();
            for entity in entities {
                models.push(entity.commit);
            }

            trace!("commits: {:?}", models);
            models
        },
        Err(e) => {
            println!("Err: {:?}", e.to_string());
            Vec::new()
        }
    }
}
