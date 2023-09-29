mod config;
mod indexer;
mod lexer;
mod util;

use config::*;
use indexer::Model;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use tiny_http::{Response, Server};

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
                        let json_path = util::str_to_path(&args.serve);

                        let model: Arc<Mutex<Model>>;
                        let exists = util::try_exists(&json_path)?;

                        if exists {
                            let m = util::load_model_from_file(&json_path)?;
                            model = Arc::new(Mutex::new(m));
                        } else {
                            model = Arc::new(Mutex::new(Default::default()));
                        }

                        {
                            let model = Arc::clone(&model);
                            let p_path = Path::new(&args.serve).to_path_buf();
                            thread::spawn(move || {
                                let mut model = model.lock().unwrap();
                                match model.walk_dir(&p_path) {
                                    Err(err) => {
                                        eprintln!("got an error: {}", err);
                                    }
                                    Ok(_) => {
                                        println!("indexed every dir");
                                        match model.save_model(&json_path) {
                                            Ok(()) => {
                                                println!(
                                                    "saved to  {path}",
                                                    path = p_path.display()
                                                );
                                            }
                                            Err(err) => {
                                                eprintln!("GOT an ERROR: {:?}", err)
                                            }
                                        }
                                    }
                                }
                            });
                        }

                        let model = Arc::clone(&model);
                        let model = model.lock().unwrap();
                        let result = model.search_query(&content);
                        let result = serde_json::to_string(&result)?;
                        //NOTE: part of the sendinkg response back to client;
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

    if !args.serve.is_empty() {
        serve(&args).unwrap_or_else(|err| {
            eprintln!("{err}");
            exit(1);
        })
    } else {
        Args::usage();
    }
}
