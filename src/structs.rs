extern crate mecab;
extern crate cabocha;

use std;

#[derive(Debug)]
pub struct Morph {
    pub surface: String,
    pub base: String,
    pub pos: String,
    pub pos1: String,
}

impl std::fmt::Display for Morph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "(surface: {}, base: {}, pos: {}, pos1: {})",
               self.surface,
               self.base,
               self.pos,
               self.pos1)
    }
}

impl Morph {
    pub fn from_mecab_node(node: mecab::Node) -> Morph {
        let mut feature = node.feature.split(",");
        Morph {
            surface: (&(node.surface)[..(node.length as usize)]).to_string(),
            pos: feature.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            pos1: feature.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            base: feature.nth(4).map(|t| t.to_string()).unwrap_or("".to_string()),
        }
    }

    pub fn from_cabocha_token(token: cabocha::Token) -> Morph {
        let mut feature_iter = token.feature_list.iter();
        Morph {
            surface: token.surface,
            pos: feature_iter.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            pos1: feature_iter.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            base: feature_iter.nth(4).map(|t| t.to_string()).unwrap_or("".to_string()),
        }
    }

    pub fn from_sentences(text: String) -> Vec<Vec<Morph>> {
        let mut parser = cabocha::Parser::new("");
        let tree = parser.parse_to_tree(text);
        let token_size = tree.token_size();

        let mut morphed_sentences = Vec::new();
        let mut morphed_sentence = Vec::new();

        for i in 0..token_size {
            let morph = Morph::from_cabocha_token(tree.token(i).unwrap());
            let is_end_of_sentence = morph.is_end_of_sentence();
            morphed_sentence.push(morph);
            if is_end_of_sentence {
                morphed_sentences.push(morphed_sentence);
                morphed_sentence = Vec::new();
            }
        }
        morphed_sentences
    }

    pub fn is_end_of_sentence(&self) -> bool {
        self.pos1 == "句点"
    }
}
