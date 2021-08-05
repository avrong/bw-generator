use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Mutex;

use axum::prelude::*;
use axum::response::Json;
use clap::Clap;
use lazy_static::lazy_static;

use bw_generator::dictionary::Dictionary;
use bw_generator::phrase::Phrase;
use bw_generator::settings::Settings;

lazy_static! {
    static ref DICTIONARY: Mutex<Option<Dictionary>> = Mutex::new(None);
}

#[tokio::main]
async fn main() {
    let settings = Settings::parse();
    let address = SocketAddr::from_str(&settings.address())
        .expect("Invalid address or port to serve on");
    let app = route("/", get(root));
    *DICTIONARY.lock().unwrap() = Some(Dictionary::from_file(&settings.dictionary));

    hyper::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Phrase> {
    let statement = DICTIONARY.lock().unwrap().as_ref().unwrap().make_statement();
    let phrase = Phrase::new(&statement);
    Json(phrase)
}