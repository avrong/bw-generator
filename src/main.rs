mod dictionary;
mod settings;

#[macro_use]
extern crate gotham_derive;

use gotham::state::{State, FromState};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single_middleware;
use gotham::pipeline::single::single_pipeline;
use crate::dictionary::{Dictionary, Phrase, DictionaryState};
use crate::settings::Settings;

fn statement_handler(state: State) -> (State, String) {
    let dictionary_state = DictionaryState::borrow_from(&state);
    let statement = dictionary_state.dictionary.lock().unwrap().make_statement();

    let phrase = Phrase::new(&statement);
    let serialized = serde_json::to_string(&phrase).unwrap();

    (state, serialized)
}

fn router(settings: &Settings) -> Router {
    let dictionary = Dictionary::from_file(&settings.dictionary_name);
    let dictionary_state = DictionaryState::new(dictionary);
    let middleware = StateMiddleware::new(dictionary_state);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.get("/").to(statement_handler)
    })
}

fn main() {
    let settings = Settings::parse();

    let address = settings.address();
    gotham::start(address, router(&settings));
}
