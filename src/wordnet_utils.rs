extern crate wordnet_stemmer;

use self::wordnet_stemmer::{WordnetStemmer, NOUN, VERB, ADJ, ADV};
use config::Config;

#[derive(Debug)]
pub enum Part {
    NOUN,
    VERB,
    ADJ,
    ADV,
}

pub fn create_wordnet_stemmter() -> Option<WordnetStemmer> {
    if let Ok(config) = Config::new() {
        WordnetStemmer::new(config.wordnet_dict_path.as_str()).ok()
    } else {
        None
    }
}

pub fn lemma(wn: &WordnetStemmer, word: String) -> (String, Option<Part>) {
    let noun_lemma = wn.lemma(NOUN, word.as_str());
    let verb_lemma = wn.lemma(VERB, word.as_str());
    let adj_lemma = wn.lemma(ADJ, word.as_str());
    let adv_lemma = wn.lemma(ADV, word.as_str());
    if word != noun_lemma { (noun_lemma, Some(Part::NOUN)) }
    else if word != verb_lemma { (verb_lemma, Some(Part::VERB)) }
    else if word != adj_lemma { (adj_lemma, Some(Part::ADJ)) }
    else if word != adv_lemma { (adv_lemma, Some(Part::ADV)) }
    else { (word, None) }
}