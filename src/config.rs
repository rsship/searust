pub struct Config {
    //NOTE: getting env variable from system ;
    pub file_path: String,
}

impl Config {
    pub fn parse(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("not enough args");
        }

        println!("{:?}", args);

        let file_path = args[0].clone();

        let config = Config { file_path };

        Ok(config)
    }
}
