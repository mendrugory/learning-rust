extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;


use hyper::{Client, Error};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use futures::{Future, future};
use std::io::{stdin};

#[derive(Debug)]
struct WikiInfo{
    title: String
}

fn main() {
     let url = get_wiki_page().unwrap();
     match request(&url){
         Ok(html) => {
             println!("Lets parse");
            parse(html)},
         Err(error) => println!("Error requesting url: {}, Error: {}", url, error)
     }
}

fn get_wiki_page() -> Result<String, std::io::Error> {
    let pre = String::from("https://en.wikipedia.org/wiki/");
    match read_input() {
        Err(error) => Err(error),
        Ok(item) => {
            let wiki_page = pre + item.trim();
            Ok(wiki_page)
        }
    }
}

fn read_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(error) => Err(error)
    }    
}

fn request(url: &str) -> Result<String, hyper::Error>{
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let uri = url.parse().unwrap();
    
    let call = client.get(uri)
        .and_then(|res| {

            res.body().fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            }).and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                future::ok::<_, Error>(s)
            })
        });
    println!("Let's scrap {}", url);
    match core.run(call){
        Ok(html) => {
            println!("{} scrapped.", url);
            Ok(html)
        },
        Err(error) => Err(error)
    }
}

fn parse(html: String){
    println!("Parsing ...");
    let dom = DOM::new(&html);
    println!("Parsed ...");
    let wiki_info = WikiInfo{
        title: dom.at("h1#firstHeading").unwrap().text_all(),
    };
    println!("{:?}", wiki_info);
}