use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct Config<'a> {
    //NOTE: getting env variable from system ;
    pub dir: &'a Path,
}

impl<'a> Config<'a> {
    pub fn parse() -> Option<Config<'a>> {
        let mut args = env::args().skip(1);
        if args.len() < 1 {
            usage();
            return None;
        }

        while let Some(arg) = args.next() {
            match &arg[..] {
                "--index" => {
                    let config = Config {
                        dir: Path::new(&arg),
                    };

                    Some(config);
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

        return None;
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
