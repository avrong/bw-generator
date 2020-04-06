use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::fs::File;
use rand::prelude::*;
use serde_derive::{Serialize, Deserialize};

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Clone, StateData)]
pub struct DictionaryState {
    pub dictionary: Arc<Mutex<Dictionary>>
}

impl DictionaryState {
    pub fn new(dictionary: Dictionary) -> DictionaryState {
        DictionaryState {
            dictionary: Arc::new(Mutex::new(dictionary))
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Dictionary {
    adverbs: Vec<String>,
    verbs: Vec<String>,
    adjectives: Vec<String>,
    nouns: Vec<String>
}

impl Dictionary {
    pub fn from_toml(data: &str) -> Dictionary {
        toml::from_str(data).expect("Cannot parse toml")
    }

    pub fn from_file(filename: &str) -> Dictionary {
        let mut file_dict = File::open(filename).expect("Cannot open file");
        let mut content_dict = String::new();
        file_dict.read_to_string(&mut content_dict).expect("Cannot read file");

        Dictionary::from_toml(&content_dict)
    }

    pub fn make_statement(&self) -> String {
        let mut rng = rand::thread_rng();

        let adverb = self.adverbs.choose(&mut rng).unwrap();
        let verb = self.verbs.choose(&mut rng).unwrap();
        let adjective = self.adjectives.choose(&mut rng).unwrap();
        let noun = self.nouns.choose(&mut rng).unwrap();

        capitalize(&format!("{} {} {} {}", adverb, verb, adjective, noun))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Phrase {
    phrase: String
}

impl Phrase {
    pub fn new(phrase: &str) -> Phrase {
        Phrase { phrase: phrase.to_owned() }
    }
}