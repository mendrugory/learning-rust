extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_native_tls;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper_native_tls::NativeTlsClient;
use hyper_native_tls::HttpsConnector;

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut res = client.get("https://google.com").send().unwrap();
    println!("{:?}", res.body());
}
