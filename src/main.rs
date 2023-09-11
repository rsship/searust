mod config;
mod lexer;
mod util;

use clap::Parser;
use lexer::*;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use util::utils::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    index: Option<Box<Path>>,

    #[arg(short, long)]
    search: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut tf_index = TermFreqIndex::new();
    let comput_tf = &mut |entry: &DirEntry| {
        let path = entry.path();

        if let Ok(content) = read_entire_file(&path) {
            let content = content.chars().collect::<Vec<_>>();
            println!("Indexing {}... ", path.display());

            let mut tf = TermFreq::new();

            for token in Lexer::new(&content) {
                let token = token
                    .into_iter()
                    .map(|x| x.to_ascii_lowercase())
                    .collect::<String>();

                if let Some(count) = tf.get_mut(&token) {
                    *count += 1;
                } else {
                    tf.insert(token, 1);
                }
            }

            let mut stats = tf.iter().collect::<Vec<_>>();
            stats.sort_by_key(|(_, f)| *f);
            stats.reverse();

            tf_index.insert(path, tf);
        } else {
            println!("unkown format: {:?}", path);
        }
    };

    let index = args.index.expect("couldn't find the index location");
    visit_dirs(&index, comput_tf)?;

    util::utils::write_tf_to_file(tf_index)?;

    Ok(())
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let first_char = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
                if first_char == '.' {
                    continue;
                }
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
