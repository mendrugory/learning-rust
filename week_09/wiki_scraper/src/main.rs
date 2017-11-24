extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;

use hyper::{Client, Chunk, Error};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use futures::{Future, future};
use std::io::{Read, stdin, stdout};

fn main() {
     let url = get_wiki_page().unwrap();
     request(url);
}

fn get_wiki_page() -> Result<String, std::io::Error> {
    let pre = String::from("https://en.wikipedia.org/wiki/");
    match read_input() {
        Err(error) => Err(error),
        Ok(item) => {
            let wiki_page = pre + &item;
            Ok(wiki_page)
        }
    }
}

fn read_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(n) => Ok(input),
        Err(error) => Err(error),
    }    
}

fn request(url: String){
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let uri = url.parse().unwrap();
    
    let call = client.get(uri)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body().fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            }).and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                println!("{}", s);
                future::ok::<_, Error>(s)
            })
        });
    println!("Let's scrap {}", url);
    match core.run(call){
        Ok(result) => println!("Good: {}", result),
        Err(error) => println!("Bad: {}", error),
    }
    println!("The End !!");   
}