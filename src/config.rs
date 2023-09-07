use std::env;

#[derive(Debug)]
pub struct Config {
    //NOTE: getting env variable from system ;
    pub file_path: String,
}

impl Config {
    pub fn parse() -> Option<Config> {
        let mut args = env::args().skip(1);
        if args.len() < 1 {
            usage();
            return None;
        }

        let mut config = Config {
            file_path: String::from(""),
        };
        while let Some(arg) = args.next() {
            match &arg[..] {
                "--index" => {
                    config.file_path = arg;
                }
                "--search" => {
                    todo!("not implementd yet");
                }
                _ => {
                    usage();
                    return None;
                }
            }
        }

        return Some(config);
    }
}

fn usage() {
    let index = format!("        {}  => used to index certain directory", "--index");
    let search = format!(
        "       {} =>  used to search on documents that indexed before ",
        "--search"
    );

    println!("\n{} \n\n {}\n", index, search);
}
