use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    let file_path = "/Users/salihbozkaya/Documents/docs.gl/gl4";

    let dir = fs::read_dir(file_path).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    for entry in dir {
        let path = entry.expect("TODO:").path();
        let content = read_entire_file(&path).unwrap_or_else(|err| {
            eprintln!("{}", err);
            exit(1);
        });

        println!("length of {:?} => {:?}", path, content.len());
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
