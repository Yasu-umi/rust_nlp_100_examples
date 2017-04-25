extern crate mecab;
extern crate cabocha;

use std;
use self::mecab::Node;
use self::cabocha::Token;

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
    pub fn from_mecab_node(node: Node) -> Morph {
        let mut feature = node.feature.split(",");
        Morph {
            surface: (&(node.surface)[..(node.length as usize)]).to_string(),
            pos: feature.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            pos1: feature.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            base: feature.nth(4).map(|t| t.to_string()).unwrap_or("".to_string()),
        }
    }

    pub fn from_cabocha_token(token: Token) -> Morph {
        let mut feature_iter = token.feature_list.iter();
        Morph {
            surface: token.surface,
            pos: feature_iter.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            pos1: feature_iter.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            base: feature_iter.nth(4).map(|t| t.to_string()).unwrap_or("".to_string()),
        }
    }

    pub fn is_end_of_sentence(&self) -> bool {
        self.pos1 == "句点"
    }
}
