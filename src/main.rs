mod config;
mod indexer;
mod lexer;
mod util;

use config::*;
use indexer::Model;
use std::env;
use std::error;
use std::fmt;
use std::path::Path;
use std::process::exit;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tiny_http::{Response, Server};

#[derive(Debug, Clone)]
struct CustomErr;

impl fmt::Display for CustomErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}", self)
    }
}

impl error::Error for CustomErr {}

fn serve(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::var("ADDRESS").unwrap_or("127.0.0.1:6969".to_string());
    let server = Server::http(&addr).unwrap();

    println!("\n serve up and runnin on {} \n", addr);

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

                        let (tx, rx) = mpsc::channel();

                        {
                            let model = Arc::clone(&model);
                            let p_path = Path::new(&args.serve).to_path_buf();
                            thread::spawn(move || {
                                let mut model = model.lock().unwrap();
                                if model.walk_dir(&p_path).is_err() {
                                    tx.send(format!("couldn't index: {}", p_path.display()))
                                        .unwrap();
                                }
                                if model.save_model(&json_path).is_err() {
                                    tx.send(format!("couldn't save: {}", p_path.display()))
                                        .unwrap();
                                }
                            });
                        }

                        for received in rx {
                            println!("{}", received);
                            exit(1);
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
