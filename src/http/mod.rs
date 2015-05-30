extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::UserAgent;

pub fn access(repo: String) {
    let mut client = Client::new();
    // let mut req = client.get("https://api.github.com/repos/rejasupotaro/kvs-schema/commits")
    let mut req = client.get("https://api.github.com")
        .header(UserAgent("cmsg".to_owned()));
    let mut res = req.send();

    match res {
        Ok(mut r) => {
            println!("Ok: {:?}", r);

            let mut body = String::new();
            r.read_to_string(&mut body).unwrap();

            println!("Response: {:?}", body);
        },
        Err(e) => println!("Err: {:?}", e.to_string())
    }
}
