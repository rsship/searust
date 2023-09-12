mod config;
mod lexer;
mod util;

use clap::Parser;
use lexer::*;
use std::fs::DirEntry;
use std::path::Path;
use std::process::exit;
use tiny_http::{Response, Server};
use util::utils::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    index: Option<Box<Path>>,

    #[arg(short, long)]
    serve: Option<String>,
}

fn indexer(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
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

    let index = args
        .index
        .as_ref()
        .expect("couldn't find the index location");
    util::utils::walk_dir(&index, comput_tf)?;

    util::utils::write_tf_to_file(tf_index, Some(true))?;
    Ok(())
}

fn serve(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Listenning on port {}", addr);
    let server = Server::http(addr).unwrap();

    for request in server.incoming_requests() {
        match request.url() {
            "/search" => {
                let response = Response::from_string("hello from search");
                request.respond(response)?;
            }
            _ => {
                let response = Response::from_string("hello world");
                request.respond(response)?;
            }
        };
    }
    Ok(())
}

fn main() {
    let args = Args::try_parse().unwrap_or_else(|err| {
        eprintln!("{err}");
        exit(1);
    });

    if args.index.is_some() {
        indexer(&args).unwrap_or_else(|err| {
            eprintln!("{err}");
            exit(1);
        })
    }

    if args.serve.is_some() {
        let addr = &args.serve.unwrap_or("0.0.0.0:6969".to_string())[..];
        serve(addr).unwrap_or_else(|err| {
            eprintln!("{err}");
            exit(1);
        })
    }
}
