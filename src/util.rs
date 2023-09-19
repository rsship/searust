pub mod utils {
    use crate::indexer::Model;
    use serde_json;
    use std::error::Error;
    use std::fs;
    use std::fs::{DirEntry, File};
    use std::io::{self, BufReader, BufWriter};
    use std::path::Path;
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

    pub fn save_model(model: &Model, pretty: Option<bool>) -> Result<(), Box<dyn Error>> {
        println!("Saving to JSON");

        let file = File::create("index.json")?;
        let pretty = pretty.unwrap_or(false);
        if pretty {
            serde_json::to_writer_pretty(BufWriter::new(file), model)?;
        } else {
            serde_json::to_writer(BufWriter::new(file), model)?;
        }
        Ok(())
    }

    pub fn walk_dir(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
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

    pub fn read_from_model(index_path: &Path) -> Result<Model, Box<dyn std::error::Error>> {
        let file = File::open(index_path)?;
        let model = serde_json::from_reader::<_, Model>(BufReader::new(file))?;

        Ok(model)
    }
}
