use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    let file_path = "/Users/salihbozkaya/Documents/docs.gl/gl3";

    let dir = fs::read_dir(file_path).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    for entry in dir {
        let path = entry.expect("TODO:").path();
        let content = read_entire_file(&path)
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(1);
            })
            .chars()
            .collect::<Vec<_>>();

        let lexer = Lexer::new(&content);

        println!("{:?} =>\n", path);
        for token in lexer {
            println!("{} \n", token)
        }

        println!("-------------------");
    }
}

struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        return Self { content };
    }

    fn trim_left(&mut self) {
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn next_token(&mut self) -> Option<String> {
        self.trim_left();

        if self.content.len() == 0 {
            return None;
        }

        let mut n = 0;

        while self.content[0].is_alphabetic() && self.content[n].is_alphanumeric() {
            n += 1;
        }

        if n == 0 {
            return None;
        }

        let token = &self.content[0..n];
        self.content = &self.content[n + 1..];

        Some(token.into_iter().collect::<String>())
    }
}
// fn next(&mut self) -> Option<Self::Item>

impl<'a> Iterator for Lexer<'a> {
    type Item = String;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.next_token()
    }
}

fn read_entire_file<P: AsRef<Path>>(file_path: P) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();

    for event in er.into_iter() {
        if let XmlEvent::Characters(event) = event? {
            content.push_str(&event);
        }
    }
    Ok(content)
}
