use super::indexer::Model;
use serde_json;
use std::error::Error;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};

pub fn read_entire_file<P: AsRef<Path>>(file_path: P) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let er = EventReader::new(BufReader::new(file));
    let mut content = String::new();

    for event in er.into_iter() {
        if let XmlEvent::Characters(event) = event? {
            content.push_str(&event);
        }
    }
    Ok(content)
}

pub fn try_exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

fn walk_dir(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let first_char = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .chars()
                .next()
                .unwrap();

            if path.is_dir() {
                if first_char == '.' {
                    continue;
                }
                walk_dir(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

pub fn load_model_from_file(index_path: &Path) -> Result<Model, Box<dyn std::error::Error>> {
    let file = File::open(index_path)?;
    let model = serde_json::from_reader::<_, Model>(BufReader::new(file))?;

    Ok(model)
}

pub fn str_to_path(index_file: &str) -> PathBuf {
    let foo = Path::new(index_file).file_name().unwrap().to_str().unwrap();
    let index_path = format!("{}.json", foo);
    let index_path = Path::new(&index_path);
    index_path.to_path_buf()
}
