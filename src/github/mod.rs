extern crate hyper;

mod commit_entity;

use std::io::Read;
use hyper::Client;
use hyper::header::UserAgent;
use rustc_serialize::json;
use self::commit_entity::CommitEntity;
use model::commit::Commit;

pub struct GitHub {
    client: Client,
}

impl GitHub {
    pub fn new() -> GitHub {
        let client = Client::new();
        GitHub { client: client }
    }

    pub fn fetch_commits(&mut self, repo: &String) -> Vec<Commit> {
        let url = format!("https://api.github.com/repos/{}/commits", repo);
        println!("fetching... {:?}", url);

        let res = self.client.get(&url)
            .header(UserAgent("cref".to_owned()))
            .send();

        match res {
            Ok(mut r) => {
                trace!("response: {:?}", r);

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
}
