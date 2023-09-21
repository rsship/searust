use super::util;

use super::lexer;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const MAX_AMOUNT: usize = 10;

type TermFreq = HashMap<String, usize>;
type DocFreq = HashMap<String, usize>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Doc {
    tf: TermFreq,
    total: usize,
    last_modified: SystemTime,
}

impl Doc {
    pub fn new(tf: TermFreq, total: usize) -> Self {
        Self {
            tf,
            total,
            last_modified: SystemTime::now(),
        }
    }
}

type Docs = HashMap<PathBuf, Doc>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub docs: Docs,
    df: DocFreq,
}

impl Model {
    pub fn new() -> Self {
        Self {
            docs: HashMap::new(),
            df: HashMap::new(),
        }
    }

    pub fn requires_reindex() {}

    pub fn remove_doc(&mut self, file_path: PathBuf) {
        if let Some(doc) = self.docs.remove(&file_path) {
            for (t, _) in doc.tf {
                if let Some(v) = self.df.get_mut(&t) {
                    *v -= 1;
                }
            }
        }
    }

    pub fn walk_dir(&mut self, dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        'looper: for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.walk_dir(&path)?;
                continue 'looper;
            }

            if let Ok(content) = util::read_entire_file(&path) {
                let content = content.chars().collect::<Vec<_>>();
                println!("Indexing {path}", path = path.display());
                self.add_doc(path, &content);
            } else {
                println!("Unknown format: {path}", path = path.display());
            }
        }
        Ok(())
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

    pub fn save_model(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(format!("{}.json", file_name))?;

        serde_json::to_writer(BufWriter::new(file), self)?;

        Ok(())
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
