use crate::lexer;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::path::PathBuf;

const MAX_AMOUNT: usize = 10;

type TermFreq = HashMap<String, usize>;
type DocFreq = HashMap<String, usize>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Doc {
    tf: TermFreq,
    total: usize,
}

impl Doc {
    pub fn new(tf: TermFreq, total: usize) -> Self {
        Self { tf, total }
    }
}

type Docs = HashMap<PathBuf, Doc>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub docs: Docs,
    pub df: DocFreq,
}

impl Model {
    pub fn new() -> Self {
        Self {
            docs: HashMap::new(),
            df: HashMap::new(),
        }
    }

    pub fn add_doc(&mut self, file_path: PathBuf, content: &[char]) {
        let mut tf = TermFreq::new();
        for token in lexer::Lexer::new(content) {
            if let Some(term) = tf.get_mut(&token) {
                *term += 1;
            } else {
                tf.insert(token, 1);
            }
        }

        for t in tf.keys() {
            if let Some(v) = self.df.get_mut(t) {
                *v += 1;
            } else {
                self.df.insert(t.to_string(), 1);
            }
        }

        self.docs.insert(file_path, Doc::new(tf, content.len()));
    }

    pub fn search_query(&self, req_val: &[char]) -> Vec<(&PathBuf, f32)> {
        let mut result = Vec::<(&PathBuf, f32)>::with_capacity(MAX_AMOUNT);
        let user_tokens = lexer::Lexer::new(req_val).collect::<Vec<_>>();

        for (path, doc) in &self.docs {
            let mut rank = 0f32;
            for token in &user_tokens {
                rank +=
                    compute_tf(&doc.tf, &token) * compute_idf(&self.df, &token, self.docs.len());
            }
            result.push((path, rank));
        }

        result.sort_by(|(_, freq1), (_, freq2)| freq2.partial_cmp(freq1).unwrap());
        result.truncate(MAX_AMOUNT);
        result
    }
}

pub fn compute_tf(tf: &TermFreq, token: &str) -> f32 {
    let n = *tf.get(token).unwrap_or(&0) as f32;
    let m = tf.into_iter().map(|(_, t)| t).sum::<usize>() as f32;
    n / m
}

pub fn compute_idf(df: &DocFreq, term: &str, n_term: usize) -> f32 {
    let n = n_term as f32;
    let m = *df.get(term).unwrap_or(&1) as f32;
    (n / m).log10()
}
