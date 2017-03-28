#!rust run

extern crate mecab;
use std;


pub struct MecabFeatureMapping {
    pub surface: String,
    pub base: String,
    pub pos: String,
    pub pos1: String,
}

impl std::fmt::Display for MecabFeatureMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "(surface: {}, base: {}, pos: {}, pos1: {})",
               self.surface,
               self.base,
               self.pos,
               self.pos1)
    }
}

pub fn feature_mappings(input: String) -> Vec<MecabFeatureMapping> {
    let mut tagger = mecab::Tagger::new("");
    let mut features: Vec<MecabFeatureMapping> = Vec::new();
    for node in tagger.parse_to_node(input).iter_next() {
        match node.stat as i32 {
            mecab::MECAB_BOS_NODE => {}
            mecab::MECAB_EOS_NODE => {}
            _ => {
                let mut feature = node.feature.split(",");
                let surface = (&(node.surface)[..(node.length as usize)]).to_string();
                let pos = feature.nth(0).map(|t| t.to_string()).unwrap_or("".to_string());
                let pos1 = feature.nth(0).map(|t| t.to_string()).unwrap_or("".to_string());
                let base = feature.nth(4).map(|t| t.to_string()).unwrap_or("".to_string());
                features.push(MecabFeatureMapping {
                    surface: surface,
                    base: base,
                    pos: pos,
                    pos1: pos1,
                });
            }
        }
    }
    features
}
