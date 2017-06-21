extern crate encoding;
extern crate regex;
extern crate wordnet_stemmer;
extern crate ndarray;
extern crate rayon;

use self::encoding::{Encoding, DecoderTrap};
use self::encoding::all::ISO_8859_1;
use self::regex::Regex;
use self::wordnet_stemmer::WordnetStemmer;
use self::ndarray::{Array1, Array2};
use self::rayon::prelude::*;

use wordnet_utils;
use config::Config;
use logistic_regression::*;

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;


pub fn create_lines_from_latin1<'a, T>(raw_texts: T)
    -> impl Iterator<Item=String> + 'a
    where T: Iterator<Item=&'a Vec<u8>> + 'a {
    raw_texts
        .filter_map(|raw_text|
            ISO_8859_1.decode(raw_text.as_slice(), DecoderTrap::Strict)
                .map(|text|
                    text.split("\n")
                        .filter(|line| !line.is_empty())
                        .map(|line| line.to_owned())
                        .collect::<Vec<String>>()
                )
                .ok()
        )
        .flat_map(|lines| lines)
}

pub fn create_word_counter_from_lines<'a, T>(lines: T)
    -> HashMap<String, u32>
    where T: Iterator<Item=&'a String> + 'a {
    let re = Regex::new(r"[,.:;-\\)\\(\?\s]").unwrap();
    let mut counter = HashMap::new();
    let terms = lines.flat_map(|line| {
            re.split(line.as_str())
                .filter(|term| !term.is_empty())
                .map(|term| term.to_owned())
                .peekable()
        });
    for term in terms {
        let value = counter.entry(term).or_insert(0u32);
        *value += 1u32;
    }
    counter
}

pub fn sorted_by_frequent_terms_from_lines<'a, T>(lines: T)
    -> Vec<(String, u32)>
    where T: Iterator<Item=&'a String> + 'a {
    let counter = create_word_counter_from_lines(lines);
    let mut count_vec: Vec<(String, u32)> = counter.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec
}

pub fn get_features_from_lines<'a, T>(wn: &'a WordnetStemmer, lines: T, stop_words: &'a HashSet<String, RandomState>)
    -> impl Iterator<Item=Vec<(String, Option<wordnet_utils::Part>)>> + 'a
    where T: Iterator<Item=&'a String> + 'a {
    let re = Regex::new(r"[,.:;-\\)\\(\?\s]").unwrap();
    lines.map(move |line|
        re.split(line.as_str())
            .filter(|&term| !term.is_empty() && !stop_words.contains(&term.to_owned().to_lowercase()))
            .map(|term| wordnet_utils::lemma(wn, term.to_owned()))
            .collect()
    )
}

pub fn get_stop_words<'a, T>(lines: T)
    -> impl Iterator<Item=String> + 'a
    where T: Iterator<Item=&'a String> + 'a {
     sorted_by_frequent_terms_from_lines(lines)
        .into_iter()
        .enumerate()
        .filter(|&(i, (ref word, count))| i < 100 || word.len() < 3 || count < 2)
        .map(|(_, (word, _))| word)
}

pub fn create_answers_iter<'a, T>(pos_lines: T, neg_lines: T)
    -> impl Iterator<Item=f32> + 'a
    where T: Iterator<Item=&'a String> + 'a {
    pos_lines.map(|_| 1f32).chain(neg_lines.map(|_| 0f32))
}

pub fn create_answers<'a, T>(pos_lines: T, neg_lines: T) -> Array1<f32>
    where T: Iterator<Item=&'a String> + 'a {
    Array1::<f32>::from_iter(create_answers_iter(pos_lines, neg_lines))
}

pub fn create_all_features<'a>(features: &'a Vec<Vec<(String, Option<wordnet_utils::Part>)>>, others_token: &'a String)
    -> Vec<&'a String> {
    let mut all_features = features.iter().flat_map(|vec| vec.iter().map(|&(ref feature, _)| feature))
        .collect::<HashSet<&'a String, RandomState>>()
        .into_iter()
        .collect::<Vec<&'a String>>();
    all_features.insert(0, others_token);
    all_features
}

pub fn create_features_vec<'a, T>(features: T, all_features: &Vec<&String>, lines_len: usize, feature_len: usize)
    -> Array2<f32>
    where T: Iterator<Item=&'a Vec<(String, Option<wordnet_utils::Part>)>> + 'a {
    let mut features_vec = Array2::<f32>::zeros((lines_len, feature_len));
    for (i, _vec) in features.enumerate() {
        let set = _vec.into_iter()
            .map(|&(ref feature, _)| feature)
            .collect::<HashSet<&String, RandomState>>();
        for (j, term) in all_features.iter().enumerate() {
            if j == 0 || set.contains(term) {
                features_vec[[i, j]] = 1f32;
            }
        }
    }
    features_vec
}

pub struct LeagningResult {
    pub feature_len: usize,
    pub all_features: Vec<String>,
    pub stop_words: HashSet<String, RandomState>,
    pub lr: LogisticRegression,
}

pub fn learning<'a>(
    wn: &'a WordnetStemmer,
    learning_data: Vec<(f32, &'a String)>,
    learning_n: usize,
    others_token: &'a String,
) -> LeagningResult {
    let learning_lines = learning_data.iter()
        .map(|&(_, line)| line.clone())
        .collect::<Vec<_>>();
    let learning_answers = Array1::<f32>::from_iter(learning_data.iter().map(|&(answer, _)| answer));

    let stop_words = get_stop_words(learning_lines.iter())
        .collect::<HashSet<String, RandomState>>();
    println!("stop words len {}", stop_words.len());

    let learning_features = get_features_from_lines(&wn, learning_lines.iter(), &stop_words)
        .collect::<Vec<_>>();
    let all_features = create_all_features(&learning_features, others_token);
    let feature_len = all_features.len();
    println!("features_vec len {}", feature_len);

    let learning_features_vec = create_features_vec(
        learning_features.iter(),
        &all_features,
        learning_lines.len(),
        feature_len
    );

    let mut lr = LogisticRegressionBuilder::new()
        .feature_len(feature_len)
        .build();
    lr.learn(&learning_features_vec, &learning_answers, learning_n);

    LeagningResult {
        feature_len: feature_len,
        all_features: all_features.into_iter().map(|feature| feature.clone()).collect::<Vec<_>>(),
        stop_words: stop_words,
        lr: lr,
    }
}

pub fn validate<'a>(
    wn: &'a WordnetStemmer,
    leagning_result: LeagningResult,
    validation_data: Vec<(f32, &'a String)>,
) -> LogisticRegressionStatics {
    let validation_lines = validation_data.iter().map(|&(_, line)| line.clone()).collect::<Vec<_>>();
    let validation_answers = Array1::<f32>::from_iter(validation_data.iter().map(|&(answer, _)| answer));
    let validation_features = get_features_from_lines(wn, validation_lines.iter(), &leagning_result.stop_words)
        .collect::<Vec<_>>();
    let learning_features_vec = create_features_vec(
        validation_features.iter(),
        &leagning_result.all_features.iter().map(|feature| feature).collect(),
        validation_lines.len(),
        leagning_result.feature_len
    );
    leagning_result.lr.get_statics(&learning_features_vec, &validation_answers)
}

pub fn k_cross_validation(k: usize, pos_lines: Vec<String>, neg_lines: Vec<String>)
    -> Vec<LogisticRegressionStatics> {
    let config = Config::new()
        .expect("Failed to load config");

    let total_len = pos_lines.len() + neg_lines.len();
    let n = if total_len % k == 0 { total_len / k } else { total_len / k + 1 };

    let mut result = Vec::with_capacity(k);
    (0..k).collect::<Vec<_>>().par_iter().map(|i| {
        let answer_line_iter = create_answers_iter(pos_lines.iter(), neg_lines.iter())
            .zip(pos_lines.iter().chain(neg_lines.iter()));

        let mut learning_data = Vec::with_capacity(n * (k-1));
        let mut validation_data = Vec::with_capacity(n);
        for (j, data) in answer_line_iter.enumerate() {
            if n * i <= j && j < n * (i+1) {
                validation_data.push(data);
            } else {
                learning_data.push(data);
            }
        }

        let wn = wordnet_utils::create_wordnet_stemmter()
            .expect("Failed to create wordnet stemmer");
        let leagning_result = learning(&wn, learning_data, 10, &config.others_token);

        validate(&wn, leagning_result, validation_data)
    }).collect_into(&mut result);
    result
}
