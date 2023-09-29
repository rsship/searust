use field_accessor::FieldAccessor;
use std::collections::HashMap;
use std::env;

#[derive(FieldAccessor, Debug)]
pub struct Args {
    pub serve: String,
}

impl Args {
    fn new_empty() -> Self {
        Args {
            serve: "".to_string(),
        }
    }
    pub fn parse() -> Self {
        let mut args_table = HashMap::<String, String>::new();
        let mut args = env::args().skip(1);
        while args.len() > 0 {
            let arg = args.next().unwrap();
            if arg.contains("--") {
                args_table.insert(arg[2..].to_string(), args.next().unwrap_or("".to_string()));
            }
        }

        let mut args = Self::new_empty();

        for (k, v) in args_table {
            if let Ok(_) = args.get(&k) {
                args.set(&k, v).unwrap();
            }
        }

        args
    }

    pub fn usage() {
        println!();
        println!("{}", "commands");
        println!("     {} {}   ", "--index", "<DIR>");
        println!("     {} {}   ", "--serve", "JSON file");
        println!();
    }
}
