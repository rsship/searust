use super::lexer;
use super::parser;
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

type Docs = HashMap<PathBuf, Doc>;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Model {
    pub docs: Docs,
    df: DocFreq,
}

impl Model {
    pub fn walk_dir(&mut self, dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        'looper: for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.walk_dir(&path)?;
                continue 'looper;
            }

            let metadata = path.metadata().map_err(|_| "couldn't get metadata")?;
            let last_modified = metadata.modified()?;

            if let Some(ext) = path.extension() {
                let ext = ext.to_str().unwrap();
                match ext {
                    "xml" | "xhtml" => {
                        parser::parse_xml(&path, |content| {
                            self.add_doc(&path, &content, last_modified).unwrap();
                        })?;
                    }
                    "pdf" => {
                        parser::parse_pdf(dir_path);
                    }
                    &_ => {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn remove_doc(&mut self, file_path: &PathBuf) {
        if let Some(doc) = self.docs.remove(file_path) {
            for (t, _) in doc.tf {
                if let Some(v) = self.df.get_mut(&t) {
                    *v -= 1;
                }
            }
        }
    }

    pub fn add_doc(
        &mut self,
        file_path: &PathBuf,
        content: &[char],
        last_modified: SystemTime,
    ) -> Result<(), ()> {
        if let Some(doc) = self.docs.get(file_path) {
            if doc.last_modified >= last_modified {
                return Ok(());
            }

            println!("file changed Reindexing {:?}", last_modified);
            self.remove_doc(file_path);
        }

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

        self.docs.insert(
            file_path.to_path_buf(),
            Doc {
                tf,
                total: content.len(),
                last_modified: last_modified,
            },
        );

        Ok(())
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

    pub fn save_model(&self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("Saving Model...");

        let file = File::create(file_path)?;
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
