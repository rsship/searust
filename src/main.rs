use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

//NOTE: add file path you wanna search for
const FILE_PATH: &str = "FILE PATH";

fn main() {
    let dir = fs::read_dir(FILE_PATH).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    // let mut collections: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut collection: HashMap<String, usize> = HashMap::new();

    for entry in dir {
        let path = entry.expect("TODO:").path();
        let content = read_entire_file(&path)
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(1);
            })
            .chars()
            .collect::<Vec<_>>();

        println!("-> {:?}", path);
        for token in Lexer::new(&content) {
            let token = token
                .into_iter()
                .map(|x| x.to_ascii_uppercase())
                .collect::<String>();

            if let Some(count) = collection.get_mut(&token) {
                *count += 1;
            } else {
                collection.insert(token, 1);
            }
        }

        println!("collection => {:?}", collection.into_iter().take(1));
        break;
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

    fn chop(&mut self, idx: usize) -> &'a [char] {
        let token = &self.content[0..idx];
        self.content = &self.content[idx..];
        return token;
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }

        return self.chop(n);
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();

        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()));
        };

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphabetic()));
        };

        return Some(self.chop(1));
    }
}
// fn next(&mut self) -> Option<Self::Item>

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
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
