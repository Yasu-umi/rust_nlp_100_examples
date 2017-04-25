extern crate mecab;
extern crate cabocha;

#[derive(Debug)]
pub struct Morph {
    pub surface: String,
    pub base: String,
    pub pos: String,
    pub pos1: String,
    pub feature: Vec<String>,
}

impl Morph {
    pub fn from_mecab_node(node: mecab::Node) -> Morph {
        let mut feature = node.feature.split(",");
        Morph {
            surface: (&(node.surface)[..(node.length as usize)]).to_string(),
            pos: feature.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            pos1: feature.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            base: feature.nth(4).map(|t| t.to_string()).unwrap_or("".to_string()),
            feature: node.feature.split(",").map(|t| t.to_string()).collect(),
        }
    }

    pub fn from_cabocha_token(token: cabocha::Token) -> Morph {
        let mut feature_iter = token.feature_list.iter();
        Morph {
            surface: token.surface,
            pos: feature_iter.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            pos1: feature_iter.nth(0).map(|t| t.to_string()).unwrap_or("".to_string()),
            base: feature_iter.nth(4).map(|t| t.to_string()).unwrap_or("".to_string()),
            feature: token.feature.split(",").map(|t| t.to_string()).collect(),
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

#[derive(Debug)]
pub struct Chunk {
    pub morphs: Vec<Morph>,
    pub dst: usize,
    pub srcs: Vec<usize>,
    pos: usize,
}

impl Chunk {
    pub fn from_cabocha_chunk(chunk: &cabocha::Chunk, tree: &cabocha::Tree, pos: &usize) -> Chunk {
        let token_range = chunk.token_pos..(chunk.token_pos + chunk.token_size);
        let morphs: Vec<Morph> = token_range
            .into_iter()
            .map(|i| Morph::from_cabocha_token(tree.token(i).unwrap()))
            .collect();
        Chunk {
            morphs: morphs,
            dst: chunk.link as usize,
            srcs: Vec::new(),
            pos: *pos,
        }
    }

    pub fn from_sentences(text: String) -> Vec<Vec<Chunk>> {
        let mut parser = cabocha::Parser::new("");
        let tree = parser.parse_to_tree(text);
        let chunk_size = tree.chunk_size();

        let mut chunked_sentences = Vec::new();
        let mut chunked_sentence = Vec::new();

        for i in 0..chunk_size {
            let cabocha_chunk = tree.chunk(i).unwrap();
            let chunk = Chunk::from_cabocha_chunk(&cabocha_chunk, &tree, &i);
            let is_end_of_sentence = chunk.is_end_of_sentence();
            chunked_sentence.push(chunk);
            if is_end_of_sentence {
                let mut tmp_chunked_sentence = Vec::new();
                let len = chunked_sentence.len();
                let dsts: Vec<usize> = chunked_sentence
                    .iter()
                    .map(|chunk| chunk.dst)
                    .collect();
                for mut chunk in chunked_sentence.into_iter() {
                    chunk.srcs = (0..len)
                        .filter(|i| *dsts.get(*i).unwrap() == chunk.pos)
                        .collect();
                    tmp_chunked_sentence.push(chunk);
                }
                chunked_sentences.push(tmp_chunked_sentence);
                chunked_sentence = Vec::new();
            }
        }
        chunked_sentences
    }

    pub fn is_end_of_sentence(&self) -> bool {
        self.morphs.iter().any(|morph| morph.is_end_of_sentence())
    }
}
