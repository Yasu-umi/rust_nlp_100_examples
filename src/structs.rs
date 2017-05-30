extern crate mecab;
extern crate cabocha;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Chunk {
    pub morphs: Vec<Morph>,
    pub dst: Option<usize>,
    pub srcs: Vec<usize>,
    pub orig_pos: usize,
    pub orig_dst: Option<usize>,
}

impl Chunk {
    fn from_cabocha_chunk(chunk: &cabocha::Chunk, tree: &cabocha::Tree, pos: &usize) -> Chunk {
        let token_range = chunk.token_pos..(chunk.token_pos + chunk.token_size);
        let morphs: Vec<Morph> = token_range.into_iter()
            .map(|i| Morph::from_cabocha_token(tree.token(i).unwrap()))
            .collect();
        let dst = if chunk.link < 0 { None } else { Some(chunk.link as usize) };
        Chunk {
            morphs: morphs,
            dst: None,
            srcs: Vec::new(),
            orig_pos: *pos,
            orig_dst: dst,
        }
    }

    pub fn from_sentences(text: String) -> Vec<Vec<Chunk>> {
        let mut parser = cabocha::Parser::new("");
        let tree = parser.parse_to_tree(text);
        let chunk_size = tree.chunk_size();

        let mut chunked_sentences = Vec::new();
        let mut chunked_sentence = Vec::new();
        let mut first_pos = 0;

        for i in 0..chunk_size {
            let cabocha_chunk = tree.chunk(i).unwrap();
            let chunk = Chunk::from_cabocha_chunk(&cabocha_chunk, &tree, &i);
            let is_end_of_sentence = chunk.is_end_of_sentence();
            chunked_sentence.push(chunk);

            if is_end_of_sentence || i == (chunk_size - 1) {
                let len = chunked_sentence.len();

                // set dst (dst is chunk pos in sentence)
                chunked_sentence = chunked_sentence.into_iter()
                    .map(|chunk| chunk.set_dst(first_pos, len))
                    .collect();

                // set srcs
                let orig_dsts: Vec<Option<usize>> = chunked_sentence.iter()
                    .map(|chunk| chunk.orig_dst)
                    .collect();
                chunked_sentence = chunked_sentence.into_iter()
                    .map(|chunk| chunk.set_srcs(&orig_dsts, len))
                    .collect();

                chunked_sentences.push(chunked_sentence);
                chunked_sentence = Vec::new();
                first_pos = i + 1;
            }
        }
        chunked_sentences
    }

    pub fn is_end_of_sentence(&self) -> bool {
        self.morphs.iter().any(|morph| morph.is_end_of_sentence())
    }

    pub fn surfaces(&self) -> String {
        self.morphs
            .iter()
            .fold(String::new(), |acc, morph| acc + morph.surface.as_str())
    }

    pub fn set_dst(mut self, first_pos: usize, sentence_len: usize) -> Chunk {
        self.dst = self.orig_dst
            .and_then(|orig_dst|
                if first_pos < orig_dst && orig_dst - first_pos < sentence_len {
                    Some(orig_dst - first_pos)
                } else {
                    None
                }
            );
        self
    }

    pub fn set_srcs(mut self, orig_dsts: &Vec<Option<usize>>, sentence_len: usize) -> Chunk {
        self.srcs = (0..sentence_len)
            .filter(|i|
                orig_dsts.get(*i)
                    .unwrap_or(&None)
                    .map(|orig_dst| orig_dst == self.orig_pos)
                    .unwrap_or(false)
            )
            .collect();
        self
    }

    pub fn morphs_of_particle(&self) -> Vec<&Morph> {
        self.morphs_of_pos("助詞")
    }

    pub fn morphs_of_pos(&self, pos: &str) -> Vec<&Morph> {
        self.morphs.iter().filter(|morph| morph.pos == pos).collect()
    }

    pub fn has_noun(&self) -> bool {
        self.include_pos("名詞")
    }

    pub fn has_verb(&self) -> bool {
        self.include_pos("動詞")
    }

    pub fn include_pos(&self, pos: &str) -> bool {
        self.morphs
            .iter()
            .any(|morph| morph.pos == pos)
    }

    pub fn include_pos1(&self, pos1: &str) -> bool {
        self.morphs
            .iter()
            .any(|morph| morph.pos1 == pos1)
    }

    pub fn to_root<'a>(&'a self, chunked_sentence: Vec<&'a Chunk>) ->  Vec<&'a Chunk> {
        let mut path_vec = Vec::new();
        let mut chunk = self;
        path_vec.push(self);
        while let Some(dst) = chunk.dst {
            if let Some(dst_chunk) = chunked_sentence.get(dst) {
                path_vec.push(dst_chunk);
                chunk = dst_chunk;
            }
        }
        path_vec
    }

    pub fn replace_noun(&self, text: &str) -> Chunk {
        let morphs = self.morphs.clone()
            .into_iter()
            .map(|morph|
                if morph.pos == "名詞" {
                    Morph {
                        surface: text.to_string(),
                        pos: morph.pos,
                        pos1: morph.pos1,
                        base: text.to_string(),
                        feature: morph.feature,
                    }
                } else {
                    morph.clone()
                }
            )
            .collect();
        Chunk {
            morphs: morphs,
            dst: self.dst,
            srcs: self.srcs.clone(),
            orig_pos: self.orig_pos,
            orig_dst: self.orig_dst,
        }
    }
}

impl PartialEq for Chunk {
    fn eq(&self, other: &Chunk) -> bool {
        self.orig_pos == other.orig_pos
    }
}
