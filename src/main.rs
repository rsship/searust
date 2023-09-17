mod config;
mod lexer;
mod util;

use config::*;
use lexer::*;
use std::fs::DirEntry;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use tiny_http::{Response, Server};
use util::utils::*;

fn compute_tf(tf: &TermFreq, token: &str) -> f32 {
    let n = *tf.get(token).unwrap_or(&0) as f32;
    let m = tf.into_iter().map(|(_, t)| t).sum::<usize>() as f32;
    n / m
}

fn compute_idf(tf_index: &TermFreqIndex, token: &str) -> f32 {
    let total_doc = tf_index.len() as f32;
    let mut count = 0;
    for (_, tf) in tf_index {
        if tf.contains_key(token) {
            count += 1;
        }
    }

    ((total_doc + 1 as f32) / count as f32).log10()
}

fn indexer(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut tf_index = TermFreqIndex::new();
    let compute_tf = &mut |entry: &DirEntry| {
        let path = entry.path();
        if let Ok(content) = read_entire_file(&path) {
            let content = content.chars().collect::<Vec<_>>();

            println!("Indexing {} ... ", path.display());

            let mut tf = TermFreq::new();

            for token in Lexer::new(&content) {
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

    util::utils::walk_dir(Path::new(&args.index), compute_tf)?;

    util::utils::write_tf_to_file(tf_index, None)?;
    Ok(())
}

fn read_tf_index(index_path: &Path) -> Result<TermFreqIndex, Box<dyn std::error::Error>> {
    let file = File::open(index_path)?;
    let tf_index = serde_json::from_reader::<_, TermFreqIndex>(BufReader::new(file))?;

    Ok(tf_index)
}

fn serve(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:6969";
    println!("\n Listenning on port  {addr} \n");
    let server = Server::http(addr).unwrap();

    for mut request in server.incoming_requests() {
        match request.method() {
            tiny_http::Method::Post => {
                match request.url() {
                    "/search" => {
                        let mut content = String::new();
                        request.as_reader().read_to_string(&mut content).unwrap();
                        let content = content.chars().collect::<Vec<_>>();
                        let lexer = Lexer::new(&content);

                        let user_tokens = lexer.collect::<Vec<_>>();
                        let tf_index = read_tf_index(Path::new(&args.serve))?;
                        let mut result = Vec::<(&Path, f32)>::new();
                        for (path, tf) in &tf_index {
                            let mut rank = 0f32;
                            for token in &user_tokens {
                                rank += compute_tf(&tf, &token) * compute_idf(&tf_index, &token);
                            }
                            println!();
                            result.push((path, rank));
                        }

                        result.sort_by(|(_, rank), (_, rank2)| rank2.partial_cmp(rank).unwrap());
                        let result = serde_json::to_string(&result)?;

                        //NOTE: part of the sending response back to client;
                        let response = Response::from_string(result).with_header(
                            "Access-Control-Allow-Origin: *"
                                .parse::<tiny_http::Header>()
                                .unwrap(),
                        );
                        request.respond(response)?;
                    }
                    _ => {
                        request.respond(Response::from_string("google is best"))?;
                    }
                };
            }
            tiny_http::Method::Get => {
                match request.url() {
                    _ => {
                        let html_file = File::open("./index.html")?;
                        request.respond(Response::from_file(html_file))?;
                    }
                };
            }
            _ => {
                println!("unknown request type");
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    if !args.index.is_empty() {
        indexer(&args).unwrap_or_else(|err| {
            eprintln!("{err}");
            exit(1);
        })
    }

    if !args.serve.is_empty() {
        serve(&args).unwrap_or_else(|err| {
            eprintln!("{err}");
            exit(1);
        })
    }
}
