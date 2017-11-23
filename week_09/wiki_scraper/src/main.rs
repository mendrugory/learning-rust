extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use std::io::Read;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let req = client.get("https://hyper.rs".parse().unwrap());
    let res = core.run(req).unwrap();
    println!("{}", res.status());

    /*let mut body = vec![];
    res.read_to_end(&mut body).unwrap();
    println!("{}", String::from_utf8_lossy(&body));*/
}