mod config;
mod indexer;
mod lexer;
mod util;

use config::*;
use indexer::Model;
use std::path::Path;
use std::process::exit;
use tiny_http::{Response, Server};

fn indexer(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut model = Model::new();
    let file_dir = Path::new(&args.index);

    if model.docs.is_empty() {
        //NOTE warm up the cache
        println!("Loading model from file");
        let file_name = file_dir.file_name().unwrap().to_str().unwrap();
        let full_file_name = format!("{}.json", file_name);
        model = util::load_model_from_file(Path::new(&full_file_name))?;
    }

    model.walk_dir(file_dir)?;

    if let Some(file_name) = file_dir.file_name() {
        model.save_model(file_name.to_str().unwrap())?;
    }

    Ok(())
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
                        let model = util::load_model_from_file(Path::new(&args.serve))?;
                        let result = model.search_query(&content);

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
                        todo!("not impelemented yet");
                    }
                };
            }
            _ => {
                todo!("not implemented yet");
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

    if args.serve.is_empty() && args.index.is_empty() {
        Args::usage();
    }
}
