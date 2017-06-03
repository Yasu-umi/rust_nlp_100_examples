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
    pub fn from_mecab_node(node: &mecab::Node) -> Morph {
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
            .filter_map(|i| tree.token(i).map(Morph::from_cabocha_token))
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

    pub fn is_end_of_sentence(&self) -> bool {
        self.morphs.iter().any(|morph| morph.is_end_of_sentence())
    }

    pub fn surfaces(&self) -> String {
        self.morphs
            .iter()
            .fold(String::new(), |acc, morph| acc + morph.surface.as_str())
    }

    pub fn set_dst(mut self, sentence_pos: usize, sentence_len: usize) -> Chunk {
        self.dst = self.orig_dst
            .and_then(|orig_dst|
                if sentence_pos < orig_dst && orig_dst - sentence_pos < sentence_len {
                    Some(orig_dst - sentence_pos)
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
                    .and_then(|orig_dst_opt| {
                        orig_dst_opt.map(|orig_dst| orig_dst == self.orig_pos)
                    })
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

    pub fn to_root_iter<'a>(&'a self, chunked_sentence: &'a Vec<Chunk>) -> ToRootChunkIter<'a> { 
        ToRootChunkIter::new(self, chunked_sentence)
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

pub struct ChunkedSentenceIter {
    tree: cabocha::Tree,
    chunk_size: usize,
    current_pos: usize,
    sentence_pos: usize,
}
impl Iterator for ChunkedSentenceIter {
    type Item = Vec<Chunk>;

    fn next(&mut self) -> Option<Vec<Chunk>> {
        let mut chunked_sentence = Vec::new();

        if self.current_pos >= self.chunk_size { return None; }

        for i in self.current_pos..self.chunk_size {
            self.current_pos += 1;
            if let Some(cabocha_chunk) = self.tree.chunk(i) {
                let chunk = Chunk::from_cabocha_chunk(&cabocha_chunk, &self.tree, &i);
                let is_end_of_sentence = chunk.is_end_of_sentence() || self.current_pos == self.chunk_size;
                chunked_sentence.push(chunk);

                if is_end_of_sentence {
                    let len = chunked_sentence.len();
                    let orig_dsts: Vec<Option<usize>> = chunked_sentence.iter()
                        .map(|chunk| chunk.orig_dst)
                        .collect();

                    chunked_sentence = chunked_sentence.into_iter()
                        .map(|chunk| chunk.set_dst(self.sentence_pos, len))
                        .map(|chunk| chunk.set_srcs(&orig_dsts, len))
                        .collect();

                    self.sentence_pos = i + 1;
                    break;
                }
            } else {
                break;
            }
        }
        Some(chunked_sentence)
    }
}

impl<'a> ChunkedSentenceIter {
    pub fn new(tree: cabocha::Tree) -> ChunkedSentenceIter {
        let chunk_size = tree.chunk_size();
        ChunkedSentenceIter {
            tree: tree,
            chunk_size: chunk_size,
            current_pos: 0,
            sentence_pos: 0,
        }
    }

    pub fn from_sentences(text: String) -> impl Iterator<Item=Vec<Chunk>> + 'a {
        ChunkedSentenceIter::new(cabocha::Parser::new("").parse_to_tree(text))
    }
}


pub struct ToRootChunkIter<'a> {
    first: bool,
    org_chunk: &'a Chunk,
    chunk: &'a Chunk,
    chunked_sentence: &'a Vec<Chunk>,
}

impl<'a> Iterator for ToRootChunkIter<'a> {
    type Item = &'a Chunk;

    fn next(&mut self) -> Option<&'a Chunk> {
        if self.first {
            self.first = false;
            Some(self.chunk)
        } else {
            self.chunk.dst.and_then(|dst|
                self.chunked_sentence.get(dst)
                    .map(|dst_chunk| {
                        self.chunk = dst_chunk;
                        dst_chunk
                    })
            )
        }
    }
}

impl<'a> Clone for ToRootChunkIter<'a> {
    fn clone(&self) -> Self {
        ToRootChunkIter {
            first: true,
            org_chunk: self.org_chunk,
            chunk: self.org_chunk,
            chunked_sentence: self.chunked_sentence,
        }
    }
}

impl<'a> ToRootChunkIter<'a> {
    pub fn new(chunk: &'a Chunk, chunked_sentence: &'a Vec<Chunk>) -> ToRootChunkIter<'a> {
        ToRootChunkIter {
            first: true,
            org_chunk: chunk,
            chunk: chunk,
            chunked_sentence: chunked_sentence,
        }
    }
}
