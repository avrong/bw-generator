use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Phrase {
    phrase: String,
}

impl Phrase {
    pub fn new(phrase: &str) -> Phrase {
        Phrase { phrase: phrase.to_owned() }
    }
}