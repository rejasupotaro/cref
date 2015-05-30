extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::UserAgent;

use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

mod commit_entity;

use self::commit_entity::CommitEntity;

pub fn fetch_commits(repo: String) {
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
            println!("Response: {:?}", entities);
        },
        Err(e) => println!("Err: {:?}", e.to_string())
    }
}
