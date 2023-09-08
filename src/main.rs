mod config;
mod lexer;
mod util;

use lexer::*;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use util::utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let config = config::Config::parse().unwrap_or_else(|| {
    //     exit(1);
    // });

    let config = config::Config {
        dir: Path::new("/home/malware/docs.gl/gl4"),
    };

    let mut tf_index = TermFreqIndex::new();

    let comput_tf = &mut |entry: &DirEntry| {
        let path = entry.path();
        let content = read_entire_file(&path)
            .expect("couldn't read the entire file")
            .chars()
            .collect::<Vec<_>>();
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
    };

    visit_dirs(config.dir, comput_tf)?;
    // let dir = fs::read_dir(config.file_path)?;
    //
    // let mut tf_index = TermFreqIndex::new();
    //
    // for entry in dir {
    //     let mut tf = TermFreq::new();
    //     let path = entry.expect("TODO:").path();
    //
    //     let content = read_entire_file(&path)?.chars().collect::<Vec<_>>();
    //
    //     println!("Indexing {}... ", path.display());
    //
    //     for token in Lexer::new(&content) {
    //         let token = token
    //             .into_iter()
    //             .map(|x| x.to_ascii_lowercase())
    //             .collect::<String>();
    //
    //         if let Some(count) = tf.get_mut(&token) {
    //             *count += 1;
    //         } else {
    //             tf.insert(token, 1);
    //         }
    //     }
    //
    //     let mut stats = tf.iter().collect::<Vec<_>>();
    //     stats.sort_by_key(|(_, f)| *f);
    //     stats.reverse();
    //
    //     tf_index.insert(path, tf);
    // }
    //
    // println!("Indexing Done...");
    //
    // write_tf_to_file(tf_index)?;

    Ok(())
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
