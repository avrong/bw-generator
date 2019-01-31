use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::net::{IpAddr, SocketAddr};
use http::Response;
use clap::{Arg, App};
use hyper::{Body, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use serde_derive::{Serialize, Deserialize};
use rand::prelude::*;

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Deserialize, Debug)]
struct Dictionary {
    adverbs: Vec<String>,
    verbs: Vec<String>,
    adjectives: Vec<String>,
    nouns: Vec<String>
}

impl Dictionary {
    fn make_statement(&self) -> String {
        let mut rng = rand::thread_rng();

        let adverb = self.adverbs.choose(&mut rng).unwrap();
        let verb = self.verbs.choose(&mut rng).unwrap();
        let adjective = self.adjectives.choose(&mut rng).unwrap();
        let noun = self.nouns.choose(&mut rng).unwrap();

        capitalize(&format!("{} {} {} {}", adverb, verb, adjective, noun))
    }
}

#[derive(Serialize, Deserialize)]
struct Phrase {
    phrase: String
}

fn main() {
    let matches = App::new("BuzzWord Generator Server")
                      .version("1.0")
                      .arg(Arg::with_name("host")
                           .short("h")
                           .long("host")
                           .value_name("TEXT")
                           .help("The interface to bind to (default: 127.0.0.1)")
                           .takes_value(true))
                      .arg(Arg::with_name("port")
                           .short("p")
                           .long("port")
                           .value_name("INTEGER")
                           .help("The port to bind to (default: 3000)")
                           .takes_value(true))
                      .arg(Arg::with_name("dictionary")
                           .short("d")
                           .long("dict")
                           .value_name("FILE")
                           .help("The buzzword dictionary to use (default: dictionary.toml)")
                           .takes_value(true))
                      .get_matches();

    let mut file_dt = File::open(matches.value_of("dictionary").unwrap_or("dictionary.toml")).unwrap();
    let mut content_dt = String::new();
    file_dt.read_to_string(&mut content_dt).unwrap();

    let dictionary: Arc<Dictionary> = Arc::new(toml::from_str(content_dt.as_str()).unwrap());

    let addr = SocketAddr::new(matches.value_of("host").unwrap_or("127.0.0.1").parse::<IpAddr>().unwrap(),
                               matches.value_of("port").unwrap_or("3000").parse::<u16>().unwrap());

    let new_svc = move || {
        let inner = Arc::clone(&dictionary);
        service_fn_ok(move |req| {
            match req.uri().path() {
                "/" => {
                    let phrase = Phrase { phrase: inner.make_statement() };
                    let serialized = serde_json::to_string(&phrase).unwrap();

                    Response::builder()
                        .header("Content-Type", "application/json; charset=utf-8")
                        .body(Body::from(serialized))
                        .unwrap()
                },
                _ => Response::builder()
                         .status(404)
                         .body(Body::from("404: Not Found"))
                         .unwrap()
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("error: {}", e));
    
    hyper::rt::run(server);
}
